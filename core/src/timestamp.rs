use rusqlite::types::{FromSql, ToSql};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp {
    stamp: i64,
}

impl ToSql for TimeStamp {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.stamp),
        ))
    }
}

impl FromSql for TimeStamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let stamp: i64 = <i64 as FromSql>::column_result(value)?;
        Ok(Self { stamp })
    }
}

impl TimeStamp {
    pub fn now() -> Self {
        let stamp = chrono::Utc::now().timestamp();
        Self { stamp }
    }
}
