//path: src\crates\logger\src\internal\logger.rs

use log::{debug, error, info, warn, Level};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn log(message: &str, level: Level) {
    INIT.call_once(|| env_logger::init());
    match level {
        Level::Info => info!("{}", message),
        Level::Warn => warn!("{}", message),
        Level::Debug => debug!("{}", message),
        Level::Error => error!("{}", message),
        _ => unimplemented!(),
    }
}
