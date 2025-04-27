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
