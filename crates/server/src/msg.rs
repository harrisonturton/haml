use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireMessage {
    jsonrpc: String,
    #[serde(flatten)]
    body: Message,
}

impl WireMessage {
    pub fn request(request: Request) -> WireMessage {
        let jsonrpc = "2.0".to_string();
        let body = Message::Request(request);
        WireMessage { jsonrpc, body }
    }

    pub fn response(response: Response) -> WireMessage {
        let jsonrpc = "2.0".to_string();
        let body = Message::Response(response);
        WireMessage { jsonrpc, body }
    }

    pub fn notification(notification: Notification) -> WireMessage {
        let jsonrpc = "2.0".to_string();
        let body = Message::Notification(notification);
        WireMessage { jsonrpc, body }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

#[derive(new, Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageId {
    I32(i32),
    String(String),
}

impl From<i32> for MessageId {
    fn from(value: i32) -> Self {
        MessageId::I32(value)
    }
}

impl From<&str> for MessageId {
    fn from(value: &str) -> Self {
        MessageId::String(value.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub id: MessageId,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Request {
    pub fn new<I, P>(id: I, method: &str, params: P) -> Request
    where
        I: Into<MessageId>,
        P: Into<Option<Value>>,
    {
        Request {
            id: id.into(),
            method: method.to_owned(),
            params: params.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: MessageId,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Response {
    pub fn success<I>(id: I, method: &str, result: Value) -> Response
    where
        I: Into<MessageId>,
    {
        Response {
            id: id.into(),
            method: method.to_owned(),
            result: Some(result),
            error: None,
        }
    }

    pub fn error<I>(id: I, method: &str, error: Error) -> Response
    where
        I: Into<MessageId>,
    {
        Response {
            id: id.into(),
            method: method.to_owned(),
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub code: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    pub fn new<V>(code: u32, message: &str, data: V) -> Error
    where
        V: Into<Option<Value>>,
    {
        Error {
            code,
            message: message.to_owned(),
            data: data.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Notification {
    pub fn new<P>(method: &str, params: P) -> Notification
    where
        P: Into<Option<Value>>,
    {
        Notification {
            method: method.to_owned(),
            params: params.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_request_with_no_params_gives_expected_json() -> TestResult {
        let req = Request::new(1, "getFoo", None);
        let msg = WireMessage::request(req);

        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"getFoo"}"#;
        let actual = serde_json::to_string(&msg)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_request_with_params_gives_expected_json() -> TestResult {
        let req = Request::new(1, "getFoo", json!({ "foo": "bar" }));
        let msg = WireMessage::request(req);

        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"getFoo","params":{"foo":"bar"}}"#;
        let actual = serde_json::to_string(&msg).unwrap();
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_success_response_gives_expected_json() -> TestResult {
        let res = Response::success(1, "getFoo", json!({ "foo": 1 }));
        let msg = WireMessage::response(res);

        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"getFoo","result":{"foo":1}}"#;
        let actual = serde_json::to_string(&msg)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_error_response_gives_expected_json() -> TestResult {
        let error = Error::new(100, "failed", None);
        let res = Response::error(1, "getFoo", error);
        let msg = WireMessage::response(res);

        let expected =
            r#"{"jsonrpc":"2.0","id":1,"method":"getFoo","error":{"code":100,"message":"failed"}}"#;
        let actual = serde_json::to_string(&msg)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_error_response_with_data_gives_expected_json() -> TestResult {
        let error = Error::new(100, "failed", json!({ "foo": "bar" }));
        let res = Response::error(1, "getFoo", error);
        let msg = WireMessage::response(res);

        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"getFoo","error":{"code":100,"message":"failed","data":{"foo":"bar"}}}"#;
        let actual = serde_json::to_string(&msg)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_notification_with_no_params_gives_expected_json() -> TestResult {
        let notification = Notification::new("getFoo", None);
        let msg = WireMessage::notification(notification);

        let expected = r#"{"jsonrpc":"2.0","method":"getFoo"}"#;
        let actual = serde_json::to_string(&msg)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_notification_with_params_gives_expected_json() -> TestResult {
        let notification = Notification::new("getFoo", json!({ "foo": "bar" }));
        let msg = WireMessage::notification(notification);

        let expected = r#"{"jsonrpc":"2.0","method":"getFoo","params":{"foo":"bar"}}"#;
        let actual = serde_json::to_string(&msg).unwrap();
        assert_eq!(actual, expected);
        Ok(())
    }
}
