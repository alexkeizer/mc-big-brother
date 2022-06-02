use tokio::sync::OnceCell;
use sqlx::{SqlitePool as DbPool, SqlitePool};
use sqlx::migrate::MigrateError;

static POOL: OnceCell<DbPool> = OnceCell::const_new();

async fn run_migrations(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}

pub async fn pool() -> &'static DbPool {
    POOL.get_or_init(|| async {
        let pool =
            SqlitePool::connect("sqlite:database.db")
                .await
                .unwrap();
        run_migrations(&pool)
            .await
            .unwrap();
        pool
    }).await
}

