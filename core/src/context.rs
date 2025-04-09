
#[derive(Debug)]
pub struct Context {
    pub db: rusqlite::Connection,
}

impl Context {
    pub fn new_testing_context() -> Result<Self, anyhow::Error> {
        let db = rusqlite::Connection::open_in_memory()?;
        mittelmeer::migrate(&db)?;
        Ok(Self { db })
    }
}
