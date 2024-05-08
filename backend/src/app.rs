use anyhow::Result;
use parking_lot::RwLock;

use crate::{
    config::Config,
    database::Db,
    writing::{self, Writing},
};

pub struct App {
    pub database: Db,
    pub config: Config,

    pub writing: RwLock<Writing>,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let connection = rusqlite::Connection::open(&config.database)?;
        let database = Db::new(connection);
        database.init()?;

        let writing = RwLock::new(writing::load(&config.writing_path)?);

        Ok(Self {
            writing,
            database,
            config,
        })
    }
}
