use std::process;

use afire::{
    trace::{self, Level},
    Middleware, Server,
};
use anyhow::Result;
use dotenvy::dotenv_override;

use app::App;
use config::Config;
use logger::{AfireLogger, RequestLogger};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
mod app;
mod config;
mod database;
mod logger;
mod routes;

fn main() -> Result<()> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    let filter = filter::Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("afire", LevelFilter::TRACE)
        .with_target("api", LevelFilter::TRACE);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenv_override().unwrap();

    let config = Config::from_env()?;

    let mut server = Server::<App>::new(&config.host, config.port)
        .workers(config.threads)
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
