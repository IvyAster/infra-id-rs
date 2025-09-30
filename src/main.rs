pub mod common;
mod handler;
mod routes;
mod services;
mod setup;
mod snowflake;

use actix_web::{App, HttpServer};
pub use common::AppResult;
use config::{Config, Environment, File};
use serde::Deserialize;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::load_config().expect("Failed to load config");
    setup::log_setup::setup(&app_config.log_config).expect("Failed to set up logging");
    let components = setup::component_setup::setup(match app_config.id_config.worker_id {
        Some(work_id) => work_id,
        _ => 0,
    });
    info!("app will run with config: {:?}", app_config.clone());
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(handler::error_handler::error_handlers())
            .configure(routes::config);
        for item in &components {
            app = app.app_data(item.clone());
        }
        app
    })
    .bind((app_config.server_config.host, app_config.server_config.port))?
    .run()
    .await
}

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    server_config: ServerConfig,
    log_config: setup::log_setup::LogConfig,
    id_config: IdConfig,
}

impl AppConfig {
    fn load_config() -> AppResult<Self> {
        let config_path = std::env::var("APP_CONFIG_PATH");
        let mut config_builder = Config::builder();
        match config_path {
            Ok(cp) => {
                config_builder =
                    config_builder.add_source(File::with_name(cp.as_str()).required(false));
            }
            _ => {
                config_builder = config_builder.add_source(File::with_name("app").required(false));
            }
        }
        let config = config_builder
            .add_source(Environment::default().separator("__"))
            .build()?;
        let app_config = config.try_deserialize()?;
        Ok(app_config)
    }
}

#[derive(Debug, Deserialize, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, Clone)]
struct IdConfig {
    worker_id: Option<u64>,
}
