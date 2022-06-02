use crate::db;

#[derive(Debug, Clone)]
pub struct Computer {
    pub id: i64,
    pub chunk_x: i64,
    pub chunk_y: i64,
}

impl Computer {
    pub async fn insert(&mut self, is_online: bool) -> Result<(), sqlx::Error> {
        let query_result = sqlx::query!(
            "INSERT INTO computer (\
                chunk_x,\
                chunk_y,\
                is_online\
            ) VALUES (?, ?, ?)",
            self.chunk_x,
            self.chunk_y,
            is_online,
        )
            .execute(db::pool().await)
            .await?;

        self.id = query_result.last_insert_rowid();

        Ok(())
    }

    pub async fn set_online(&self, is_online: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE computer
             SET is_online = ?
             WHERE id = ?",
            is_online,
            self.id,
        )
            .execute(db::pool().await)
            .await?;
        Ok(())
    }

    // Sets the online status of all computers
    pub async fn set_online_all(is_online: bool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE computer SET is_online = ?", is_online)
            .execute(db::pool().await)
            .await?;

        Ok(())
    }
}