#![allow(dead_code)]

use std::{env, io, str::FromStr, sync::Once};

use defer_lite::defer;
pub use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;

#[cfg(debug_assertions)]
const DEBUG_ASSERTIONS: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG_ASSERTIONS: bool = false;

pub type FileLogGuard = WorkerGuard;

static ONCE: Once = Once::new();

pub fn init_log(filelog: Option<bool>) -> Option<FileLogGuard> {
    if ONCE.is_completed() {
        return None;
    }
    ONCE.call_once(|| {});
    defer! {info!("logger initialized")}

    // Set the format of the log output,
    // for example, whether to include the log level,
    // whether to include the location of the log source,
    // and set the time format of the log.
    // via: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format().with_level(true).with_target(true);

    // max log level
    let max_level = env::var("RUST_LOG")
        .map_or_else(|_| Level::INFO, |s| Level::from_str(&s).unwrap_or(Level::INFO));

    // Initialize and set the log format (customize and filter logs)
    let subscriber_builder =
        tracing_subscriber::fmt().with_max_level(max_level).event_format(format);

    if !filelog.unwrap_or(!DEBUG_ASSERTIONS) {
        subscriber_builder
            .with_writer(io::stdout)
            .init();
        return None;
    }

    // Use tracing_appender to specify the output destination of the log
    // via: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    let file_appender = tracing_appender::rolling::daily("./.out/log/", "anfs.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    subscriber_builder
        .with_writer(non_blocking)// write to file, will overwrite stdout above
        .with_ansi(false)// If the log is written to a file, the ansi color output function should be turned off
        .init();
    Some(guard)
}

#[test]
fn test() {
    use tracing::*;

    #[allow(unused_variables)]
        //let x = init_log(None);
        #[allow(unused_variables)]
        let x = init_log(None);

    let warn_description = "Invalid Input";
    let input = &[0x27, 0x45];

    warn!(?input, warning = warn_description);
    warn!(target: "evmmmmmm",warning2 = warn_description, "Received warning for input: {:?}", input,);
}
