use std::time::SystemTime;
use sqlx::SqlitePool;
use crate::{Computer, ComputerData, ComputerId};

pub struct ComputerRepo {
    pool: &'static SqlitePool,
}

impl ComputerRepo {
    pub fn new(pool: &'static SqlitePool) -> Self {
        Self { pool }
    }

    /// Does an upsert, either retrieving an existing computer with the given `(world, dimension,
    /// pos_x, pos_z)` combination (updating the version in the process), or adds a new row with the
    /// given data.
    pub async fn upsert(&self, data: ComputerData) -> anyhow::Result<Computer> {
        let row = sqlx::query!(
            "
INSERT INTO computers (version, world, dimension, pos_x, pos_z)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT
    DO UPDATE SET version=excluded.version
RETURNING id;
            ",
            data.version,
            data.world.0,
            data.dimension.0,
            data.pos_x,
            data.pos_z,
            )
            .fetch_one(self.pool)
            .await?;

        Ok(Computer {
            id: ComputerId::new(row.id as u32),
            data,
        })
    }

    async fn insert_ping_(&self, id: ComputerId) -> anyhow::Result<()> {
        let time =
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_millis();

        let [time_hi, time_lo]: [i64; 2] = bytemuck::cast(time);
        let id = id.inner();

        sqlx::query!(
            "
INSERT INTO ping (computer, time_hi, time_lo)
    VALUES (?, ?, ?)
            ",
            id,
            time_hi,
            time_lo,
            )
            .execute(self.pool)
            .await?;

        Ok(())
    }


    #[inline]
    /// Inserts a new row into the ping table, for the given computer, with the current (system) time
    pub async fn insert_ping(&self, id: impl Into<ComputerId>) -> anyhow::Result<()> {
        self.insert_ping_(id.into()).await
    }
}