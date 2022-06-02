use std::collections::HashSet;

use tokio::sync::Mutex;

use crate::Computer;

mod run_tcp;

pub struct Server {
    online_computers: Mutex<HashSet<Computer>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            online_computers: Default::default()
        }
    }
}