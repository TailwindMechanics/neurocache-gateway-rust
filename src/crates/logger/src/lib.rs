//path: src\crates\logger\src\lib.rs

use log::Level;
mod internal;

pub fn log(message: &str) {
    internal::logger::log(message, Level::Debug);
}

pub fn log_info(message: &str) {
    internal::logger::log(message, Level::Info);
}

pub fn log_warn(message: &str) {
    internal::logger::log(message, Level::Warn);
}

pub fn log_error(message: &str) {
    internal::logger::log(message, Level::Error);
}
