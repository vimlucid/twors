use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum Error {
    #[error("Something went wrong: {0}")]
    Generic(String),
}

impl Error {
    pub fn from_string(message: String) -> Error {
        Error::Generic(message)
    }

    pub fn from_str(message: &str) -> Error {
        Error::Generic(message.to_owned())
    }
}

impl From<Error> for JsValue {
    fn from(val: Error) -> Self {
        JsValue::from_str(&format!("{val}"))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
