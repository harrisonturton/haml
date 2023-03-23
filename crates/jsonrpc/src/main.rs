use serde::Deserialize;
use serde_json::json;

pub mod msg;
pub mod router;

use msg::Request;
use router::Router;

fn main() {
    let router = Router::new()
        .register("getFoo", get_foo)
        .register("getBar", get_bar);

    let params = json!({ "foo_id": "my-bar" });
    let req = Request::new(1, "getFoo", params);
    router.run("getFoo", req);

    let params = json!({ "bar_id": "my-bar" });
    let req = Request::new(1, "getBar", params);
    router.run("getBar", req);
}

#[derive(Debug, Deserialize)]
pub struct GetFooRequest {
    pub foo_id: String,
}

fn get_foo(req: GetFooRequest) {
    println!("Getting foo: {}", req.foo_id);
}

#[derive(Debug, Deserialize)]
pub struct GetBarRequest {
    pub bar_id: String,
}

fn get_bar(req: GetBarRequest) {
    println!("Getting bar: {}", req.bar_id);
}
