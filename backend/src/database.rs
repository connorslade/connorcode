use std::process;

use anyhow::Result;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use rusqlite::{params, Connection};
use tracing::{error, info};

// Increment every time schema changes in a non backwards compatible way, even in dev
const DATABASE_VERSION: u64 = 1;

pub struct Db {
    inner: Mutex<Option<Connection>>,
}

impl Db {
    pub fn new(connection: Connection) -> Self {
        Self {
            inner: Mutex::new(Some(connection)),
        }
    }

    fn take(&self) -> Connection {
        let val = self.inner.lock().take();
        val.expect("No value to take")
    }

    fn lock(&self) -> MappedMutexGuard<'_, Connection> {
        MutexGuard::map(self.inner.lock(), |x: &mut Option<Connection>| {
            x.as_mut().expect("No value to take")
        })
    }
}

impl Db {
    pub fn init(&self) -> Result<()> {
        let mut this = self.lock();
        this.pragma_update(None, "journal_mode", "WAL")?;
        this.pragma_update(None, "synchronous", "NORMAL")?;

        let db_version =
            this.pragma_query_value(None, "user_version", |row| row.get::<_, u64>(0))?;

        match db_version {
            DATABASE_VERSION => info!("Loaded database at `{}`", this.path().unwrap()),
            0 => {
                info!("Creating database at `{}`", this.path().unwrap());
                this.pragma_update(None, "user_version", DATABASE_VERSION)?;
            }
            i => {
                error!(
                    "Database version mismatch. Expected {}, got {}. Please run migrations, or \
                     just like delete the database and start over.",
                    DATABASE_VERSION, i
                );
                drop(this);
                self.cleanup()?;
                process::exit(1);
            }
        }

        let trans = this.transaction()?;
        for i in [
            include_str!("sql/init_analytics.sql"),
            include_str!("sql/init_legacy_views.sql"),
        ] {
            trans.execute(i, [])?;
        }
        trans.commit()?;

        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        let this = self.take();
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        this.pragma_update(None, "optimize", "")?;
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        Ok(())
    }
}

impl Db {
    pub fn insert_analytics(&self, data: crate::routes::analytics::Analytics) -> Result<()> {
        let ip = u32::from_be_bytes(data.ip.octets());
        let page = if data.page.starts_with('/') {
            &data.page[1..]
        } else {
            &data.page
        };

        self.lock().execute(
            "INSERT INTO analytics VALUES (?, ?, ?, ?, ?)",
            params![data.timestamp, ip, page, data.referrer, data.user_agent],
        )?;
        Ok(())
    }

    pub fn get_view_count(&self, page: &str) -> Result<u32> {
        const VIEW_TIMEOUT: u32 = 60 * 20;
        let count = self.lock().query_row(
            include_str!("sql/view_count.sql"),
            params![VIEW_TIMEOUT, page],
            |row| row.get::<_, u32>(0),
        )?;
        Ok(count)
    }
}
