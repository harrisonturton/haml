use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::msg::Request;

#[derive(Debug)]
pub struct Server {
    pub addr: String,
    pub threads: u32,
}

pub struct Router {
    pub methods: HashMap<String, Box<dyn MethodHandler>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            methods: HashMap::new(),
        }
    }

    pub fn register<T>(mut self, name: &str, handler: impl Fn(T) + 'static) -> Self
    where
        T: DeserializeOwned + 'static,
    {
        self.methods.insert(
            name.to_owned(),
            Box::new(CallbackMethodHandler {
                handler: Box::new(handler),
            }),
        );
        self
    }

    pub fn run(&self, method: &str, message: Request) {
        let handler = self.methods.get(method).unwrap();
        handler.call(message.params.unwrap());
    }
}

pub trait MethodHandler {
    fn call(&self, req: Value);
}

pub struct CallbackMethodHandler<T: DeserializeOwned> {
    pub handler: Box<dyn Fn(T)>,
}

impl<T> MethodHandler for CallbackMethodHandler<T>
where
    T: DeserializeOwned,
{
    fn call(&self, value: Value) {
        let value = serde_json::from_value(value).unwrap();
        (self.handler)(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;
    use std::{cell::Cell, rc::Rc};

    #[derive(Debug, Deserialize)]
    pub struct GetFooRequest {
        pub foo_id: String,
    }

    #[test]
    fn test_calls_expected_method() {
        // This flag is set by the callback. Since we need to mutate it inside
        // the closure, and then check it again outside the closure, we need
        // shared mutability.
        let invoked_callback = Rc::new(Cell::new(false));

        let handler_flag = invoked_callback.clone();
        let handler = move |_req: GetFooRequest| {
            handler_flag.clone().set(true);
        };

        let router = Router::new().register("getFoo", handler);

        let params = json!({ "foo_id": "my-foo" });
        let req = Request::new(1, "getFoo", params);
        router.run("getFoo", req);

        assert!(invoked_callback.get());
    }
}
