use super::Migration;

pub struct Migration000 {}

impl Migration for Migration000 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE todos (
                id          BIGINT NOT NULL UNIQUE AUTOINCREMENT,
                title       TEXT NOT NULL,
                body        TEXT NOT NULL,
                scheduled   TEXT NOT NULL,
                deadline    TEXT NOT NULL,
                state       INTEGER NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        todo!()
    }
}
