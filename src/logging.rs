use log::LevelFilter;

/// Returns the log level defined in the env.
/// If not defined, returns `log::LevelFilter::info`.
pub fn log_level() -> LevelFilter {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".into());

    match log_level.to_uppercase().as_str() {
        "OFF" => LevelFilter::Off,
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        "DEBUG" | "DBG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        // By default, Info
        _ => LevelFilter::Info,
    }
}
