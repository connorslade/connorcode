use anyhow::Result;

use crate::{
    config::Config,
    database::Db,
    writing::{self, Writing},
};

pub struct App {
    pub database: Db,
    pub config: Config,

    pub writing: Writing,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let connection = rusqlite::Connection::open(&config.database)?;
        let database = Db::new(connection);
        database.init()?;

        Ok(Self {
            database,
            config,
            writing: writing::load()?,
        })
    }
}
