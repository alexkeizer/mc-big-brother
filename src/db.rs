mod computer;
pub(crate) mod models;

use sqlx::SqlitePool;
use tokio::sync::OnceCell;

pub use computer::ComputerRepo;

// TODO: make this resilient to database restarts (is that even a thing with sqlite?)
static POOL: OnceCell<SqlitePool> = OnceCell::const_new();


pub async fn connection() -> anyhow::Result<&'static SqlitePool> {
    let pool = POOL.get_or_try_init(|| SqlitePool::connect("db/skydaddy.db")).await?;

    Ok(pool)
}