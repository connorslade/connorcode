use std::process;

use afire::{
    trace::{self, Level},
    Middleware, Server,
};
use anyhow::Result;
use dotenvy::dotenv_override;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use app::App;
use config::Config;
use logger::{AfireLogger, RequestLogger};
mod app;
mod config;

mod database;
mod logger;
mod markdown;
mod misc;
mod routes;
mod writing;

fn main() -> Result<()> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    let filter = filter::Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("afire", LevelFilter::TRACE)
        .with_target("backend", LevelFilter::TRACE);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = dotenv_override() {
        warn!("Failed to load .env: {}", e);
    }

    let config = Config::from_env()?;

    let mut server = Server::<App>::new(&config.host, config.port)
        .workers(config.threads)
        .keep_alive(config.keep_alive)
        .state(App::new(config)?);

    RequestLogger.attach(&mut server);
    routes::attach(&mut server);

    let shutdown_state = server.app();
    ctrlc::set_handler(move || {
        info!("Shutting down");
        shutdown_state.database.cleanup().unwrap();
        info!("Database closed.");
        process::exit(0);
    })?;

    server.run()?;
    Ok(())
}
