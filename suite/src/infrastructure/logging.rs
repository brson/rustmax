// Logging infrastructure with structured output.

use rmx::prelude::*;
use std::io::Write;
use std::collections::HashMap;

// Initialize logging system.
pub fn init_logging(level: &str) -> AnyResult<()> {
    let mut builder = rmx::env_logger::Builder::from_default_env();

    // Parse log level.
    let level = match level.to_lowercase().as_str() {
        "trace" => rmx::log::LevelFilter::Trace,
        "debug" => rmx::log::LevelFilter::Debug,
        "info" => rmx::log::LevelFilter::Info,
        "warn" => rmx::log::LevelFilter::Warn,
        "error" => rmx::log::LevelFilter::Error,
        _ => rmx::log::LevelFilter::Info,
    };

    builder
        .filter_level(level)
        .format(|buf, record| {
            use rmx::chrono::Local;

            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    rmx::log::info!("Logging initialized at level: {}", level);
    Ok(())
}

// Structured log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, rmx::serde_json::Value>,
}

impl LogEntry {
    pub fn new(level: &str, target: &str, message: impl Into<String>) -> Self {
        use rmx::chrono::Utc;

        Self {
            timestamp: Utc::now().to_rfc3339(),
            level: level.to_string(),
            target: target.to_string(),
            message: message.into(),
            fields: std::collections::HashMap::new(),
        }
    }

    pub fn field(mut self, key: impl Into<String>, value: impl rmx::serde::Serialize) -> Self {
        if let Ok(value) = rmx::serde_json::to_value(value) {
            self.fields.insert(key.into(), value);
        }
        self
    }

    pub fn log(self) {
        // let json = rmx::serde_json::to_string(&self).unwrap_or_else(|_| self.message.clone());
        let json = self.message.clone(); // Simplified for now
        match self.level.as_str() {
            "trace" => rmx::log::trace!("{}", json),
            "debug" => rmx::log::debug!("{}", json),
            "info" => rmx::log::info!("{}", json),
            "warn" => rmx::log::warn!("{}", json),
            "error" => rmx::log::error!("{}", json),
            _ => rmx::log::info!("{}", json),
        }
    }
}

// Performance timer for measuring operations.
pub struct PerfTimer {
    name: String,
    start: std::time::Instant,
}

impl PerfTimer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let elapsed = self.elapsed_ms();
        if elapsed > 1000 {
            rmx::log::warn!("Operation '{}' took {}ms", self.name, elapsed);
        } else {
            rmx::log::debug!("Operation '{}' took {}ms", self.name, elapsed);
        }
    }
}

// Macro for structured logging with fields.
#[macro_export]
macro_rules! log_struct {
    ($level:ident, $target:expr, $msg:expr $(, $key:expr => $value:expr)*) => {
        {
            let entry = $crate::infrastructure::logging::LogEntry::new(
                stringify!($level),
                $target,
                $msg
            );
            $(
                let entry = entry.field($key, $value);
            )*
            entry.log();
        }
    };
}