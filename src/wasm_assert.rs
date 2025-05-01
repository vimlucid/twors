/// This will output an assertion failure message (with the failed expression) via the logger
/// before panicking - this results in a readable error message in the browser.
#[macro_export]
macro_rules! wasm_assert {
    ($cond:expr) => {
        if (!($cond)) {
            let message = format!("Assertion failed: {}", stringify!($cond));
            log::error!("{}", message);
            panic!("{}", message);
        }
    };
}
