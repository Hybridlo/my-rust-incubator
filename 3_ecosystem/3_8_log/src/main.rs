use std::fs;

use tracing::{Level, info, warn};
use tracing_subscriber::{prelude::*, fmt::{format::{Format, JsonFields}, Layer}, filter::Targets};

fn register_loggers() {
    let stderr = std::io::stderr.with_max_level(Level::WARN);
    let file_handle = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("access.log")
        .expect("To be able to open log file");

    let local_layer = Layer::new()
        .event_format(
            Format::default()
                .json()
                .flatten_event(true)
            )
        .fmt_fields(JsonFields::new())
        .with_writer(file_handle)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_filter(Targets::new().with_target("local", Level::DEBUG));

    let global_layer = Layer::new()
        .event_format(
            Format::default()
                .json()
                .flatten_event(true)
        )
        .fmt_fields(JsonFields::new())
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .map_writer(move |w| stderr.or_else(w))
        .with_target(false);

    tracing_subscriber::registry()
        .with(local_layer)
        .with(global_layer)
        .init();  
}

fn main() {
    register_loggers();

    info!(msg = "Logging some info");

    warn!(msg = "post msg", method = "POST", path = "/app");

    warn!(target: "local", msg = "file_log");
}
