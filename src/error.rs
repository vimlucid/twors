use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum Error {
    #[error("Something went wrong: {0}")]
    Generic(String),
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Generic(message.to_owned())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Generic(message)
    }
}

impl From<Error> for JsValue {
    fn from(val: Error) -> Self {
        JsValue::from_str(&format!("{val}"))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
