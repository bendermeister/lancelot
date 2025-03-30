use super::Migration;

pub struct Migration000 {}

impl Migration for Migration000 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE todos (
                id          INTEGER NOT NULL UNIQUE,
                title       TEXT NOT NULL,
                body        TEXT NOT NULL,
                scheduled   TEXT,
                deadline    TEXT,
                state       INTEGER NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
