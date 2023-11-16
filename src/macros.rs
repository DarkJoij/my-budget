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

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        casual_logger::Log::trace(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        casual_logger::Log::info(&format!($($arg)*))
    }
}


#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => {
        casual_logger::Log::notice(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        casual_logger::Log::warn(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let message = &format!($($arg)*);

        casual_logger::Log::fatal(&message);
        panic!("unhandled: {}", message);
    }}
}
