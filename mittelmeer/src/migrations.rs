use super::Migration;

pub struct Migration000 {}

impl Migration for Migration000 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE todos (
                id          INTEGER NOT NULL UNIQUE,
                title       TEXT NOT NULL,
                path        TEXT NOT NULL,
                scheduled   INTEGER,
                deadline    INTEGER,
                opened      INTEGER NOT NULL,
                closed      INTEGER,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
