//path: src\crates\logger\src\lib.rs

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        println!("{}", $msg);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => ({
        println!("{}", $msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        println!(concat!("INFO: ", $fmt), $($arg)*);
    });
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => ({
        println!("{}", $msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        println!(concat!("DEBUG: ", $fmt), $($arg)*);
    });
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => ({
        println!("{}", $msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        println!(concat!("WARN: ", $fmt), $($arg)*);
    });
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => ({
        eprintln!("{}", $msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!(concat!("ERROR: ", $fmt), $($arg)*);
    });
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => ({
        println!("{}", $msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        println!(concat!("TRACE: ", $fmt), $($arg)*);
    });
}
