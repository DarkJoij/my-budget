#[macro_export]
macro_rules! if_ultimate_version {
    ($ultimate_block:block else $else_block:block) => {{
        if cfg!(feature = "ultimate") {
            $ultimate_block
        } else {
            $else_block
        }
    }};
    ($($block:tt)*) => {{
        if cfg!(feature = "ultimate") {
            $($block)*
        }
    }};
}
