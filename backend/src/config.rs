use std::{env, path::PathBuf};

use anyhow::{Context, Result};

pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub keep_alive: bool,
    pub database: PathBuf,
    pub external_url: String,

    pub redirect_path: PathBuf,
    pub files_path: PathBuf,
    pub writing_path: PathBuf,
}

fn env(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("Environment variable not set: {key}"))
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env("HOST")?,
            port: env("PORT")?.parse()?,
            threads: env("THREADS")?.parse()?,
            keep_alive: env("KEEP_ALIVE")?.parse()?,
            database: env("DATABASE")?.into(),
            external_url: env("EXTERNAL_URL")?,

            redirect_path: env("REDIRECT_PATH")?.into(),
            files_path: env("FILES_PATH")?.into(),
            writing_path: env("WRITING_PATH")?.into(),
        })
    }
}
