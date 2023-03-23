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

#[cfg(tests)]
mod tests {
    use super::*;
}
