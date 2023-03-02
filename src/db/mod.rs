pub mod models;
//pub mod schema;
use std::path::Path;
use std::sync::Arc;

pub use models::*;
use once_cell::sync::OnceCell;
pub use sqlx::query_as;
use sqlx::{query, Connection, Error, SqliteConnection};
use tokio::sync::Mutex;

pub struct DB {
    internal: OnceCell<Arc<Mutex<SqliteConnection>>>,
}

impl DB {
    pub const fn empty() -> Self {
        DB {
            internal: OnceCell::new(),
        }
    }

    pub async fn init<P: AsRef<Path>>(&self, path: P) -> Result<(), sqlx::Error> {
        if self
            .internal
            .set(Arc::new(Mutex::new(
                SqliteConnection::connect(&path.as_ref().display().to_string()).await?,
            )))
            .is_err()
        {
            panic!("Init can be called only once")
        }

        Ok(())
    }

    /// Returns underlying DB instance guarded with Mutex.
    ///
    /// # Panics
    ///
    /// Panics if db is not initialized of poisoned
    pub async fn db(&self) -> tokio::sync::MutexGuard<SqliteConnection> {
        self.internal
            .get()
            .expect("DB should be initialized by now")
            .lock()
            .await
    }

    pub async fn query<S: AsRef<str>>(
        &self,
        q: S,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, Error> {
        query(q.as_ref()).execute(&mut (*self.db().await)).await
    }

    /// Create tables if they do not exists
    pub async fn create(&self) -> Result<(), Error> {
        let db = &mut (*self.db().await);
        query(
            "CREATE TABLE IF NOT EXISTS pull (
                repo TEXT NOT NULL,
                num INTEGER NOT NULL,
                status TEXT NOT NULL,
                merge_sha TEXT,
                title TEXT,
                body TEXT,
                head_sha TEXT,
                head_ref TEXT,
                base_ref TEXT,
                assignee TEXT,
                approved_by TEXT,
                priority INTEGER,
                try_ INTEGER,
                rollup INTEGER,
                squash INTEGER,
                delegate TEXT,
                UNIQUE (repo, num)
            )",
        )
        .execute(&mut *db)
        .await?;
        query(
            "CREATE TABLE IF NOT EXISTS build_res (
                repo TEXT NOT NULL,
                num INTEGER NOT NULL,
                builder TEXT NOT NULL,
                res INTEGER,
                url TEXT NOT NULL,
                merge_sha TEXT NOT NULL,
                UNIQUE (repo, num, builder)
            )",
        )
        .execute(&mut *db)
        .await?;
        query(
            "CREATE TABLE IF NOT EXISTS mergeable (
                repo TEXT NOT NULL,
                num INTEGER NOT NULL,
                mergeable INTEGER NOT NULL,
                UNIQUE (repo, num)
            )",
        )
        .execute(&mut *db)
        .await?;
        query(
            "CREATE TABLE IF NOT EXISTS repos (
                repo TEXT NOT NULL,
                treeclosed INTEGER NOT NULL,
                treeclosed_src TEXT,
                UNIQUE (repo)
            )",
        )
        .execute(&mut *db)
        .await?;
        query(
            "CREATE TABLE IF NOT EXISTS retry_log (
                repo TEXT NOT NULL,
                num INTEGER NOT NULL,
                time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                src TEXT NOT NULL,
                msg TEXT NOT NULL
            )",
        )
        .execute(&mut *db)
        .await?;
        query(
            "CREATE INDEX IF NOT EXISTS retry_log_time_index ON retry_log
            (repo, time DESC)",
        )
        .execute(&mut *db)
        .await?;

        // manual DB migration :/

        if matches!(
            query("SELECT treeclosed_src FROM repos LIMIT 0")
                .execute(&mut *db)
                .await,
            Err(Error::Database(_))
        ) {
            query("ALTER TABLE repos ADD COLUMN treeclosed_src TEXT")
                .execute(&mut *db)
                .await?;
        }

        if matches!(
            query("SELECT squash FROM pull LIMIT 0")
                .execute(&mut *db)
                .await,
            Err(Error::Database(_))
        ) {
            query("ALTER TABLE pull ADD COLUMN squash INT")
                .execute(&mut *db)
                .await?;
        }

        Ok(())
    }
}
