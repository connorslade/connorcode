use std::{env, path::PathBuf};

use anyhow::Result;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub database: PathBuf,
    pub external_url: String,

    pub files_path: PathBuf,
    pub writing_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env::var("HOST")?,
            port: env::var("PORT")?.parse()?,
            threads: env::var("THREADS")?.parse()?,
            database: env::var("DATABASE")?.into(),
            external_url: env::var("EXTERNAL_URL")?,

            files_path: env::var("FILES_PATH")?.into(),
            writing_path: env::var("WRITING_PATH")?.into(),
        })
    }
}
