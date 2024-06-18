use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use parking_lot::RwLock;

use crate::{
    config::Config,
    database::Db,
    writing::{self, Writing},
};

pub struct App {
    pub database: Db,
    pub config: Config,

    pub redirects: HashMap<String, String>,
    pub writing: RwLock<Writing>,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let connection =
            rusqlite::Connection::open(&config.database).context("Opening database")?;
        let database = Db::new(connection);
        database.init().context("Initializing database")?;

        let raw_redirects =
            fs::read_to_string(&config.redirect_path).context("Reading redirects file")?;
        let redirects = toml::from_str(&raw_redirects).context("Parsing redirects file")?;

        let writing = RwLock::new(writing::load(&config.writing_path).context("Loading articles")?);

        Ok(Self {
            redirects,
            writing,
            database,
            config,
        })
    }
}
