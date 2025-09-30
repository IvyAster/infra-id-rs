use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::io;
use serde::Deserialize;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::AppResult;

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    level: String,
    location: String,
    file_prefix: String,
    rotation: Option<String>,
    appender: Option<String>,
}

impl LogConfig {
    fn get_level(&self) -> Level {
        self.level.parse::<Level>().unwrap_or(Level::INFO)
    }
    fn get_rotation(&self) -> Rotation {
        match &self.rotation {
            Some(ro) => {
                match ro.as_str() {
                    "daily" => Rotation::DAILY,
                    "never" => Rotation::NEVER,
                    "minute" => Rotation::MINUTELY,
                    "hour" => Rotation::HOURLY,
                    _ =>  Rotation::NEVER,
                }
            }
            _ => Rotation::NEVER,
        }
    }
}

macro_rules! json_layer {
    ($writer:expr) => {
        fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_writer($writer)
    };
}

pub fn setup(log_config: &LogConfig) -> AppResult<()> {
    let log_level = log_config.get_level();
    let env_filter = EnvFilter::from_default_env().add_directive(log_level.into());

    match log_config.appender.as_deref() {
        Some("file") => {
            let file_appender = RollingFileAppender::new(
                log_config.get_rotation(),
                &log_config.location,
                &log_config.file_prefix,
            );

            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(file_appender))
                .init();
        }
        Some("all") => {
            let file_appender = RollingFileAppender::new(
                log_config.get_rotation(),
                &log_config.location,
                &log_config.file_prefix,
            );

            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(file_appender))
                .with(json_layer!(io::stdout))
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(io::stdout))
                .init();
        }
    }

    Ok(())
}