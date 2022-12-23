use crate::db;

mod run_tcp;

pub struct Server {
    computer_repo: db::ComputerRepo
}

impl Server {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = db::connection().await?;
        Ok(Self {
            computer_repo: db::ComputerRepo::new(pool)
        })
    }
}