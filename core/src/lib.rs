use chrono::NaiveDate;
use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ID<T> {
    id: i64,
    marker: PhantomData<T>,
}

impl<T> ID<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

impl<T> std::default::Default for ID<T> {
    fn default() -> Self {
        Self {
            id: 0,
            marker: PhantomData,
        }
    }
}

impl<T> Deref for ID<T> {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T> DerefMut for ID<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}

impl<T> FromSql for ID<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let id: i64 = <i64 as FromSql>::column_result(value)?;
        Ok(Self {
            id,
            marker: PhantomData,
        })
    }
}

impl<T> ToSql for ID<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.id.to_sql()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TodoState {
    Todo,
    Done,
}

impl ToSql for TodoState {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let state: i64 = match self {
            TodoState::Todo => 0,
            TodoState::Done => 1,
        };
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(state)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub id: Option<ID<Todo>>,
    pub scheduled: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub state: TodoState,
}

impl Todo {
    pub fn new(
        title: String,
        body: String,
        scheduled: Option<NaiveDate>,
        deadline: Option<NaiveDate>,
    ) -> Self {
        Self {
            title,
            body,
            id: None,
            scheduled,
            deadline,
            state: TodoState::Todo,
        }
    }

    pub fn fetch_all(db: &rusqlite::Connection) -> Result<Vec<Self>, anyhow::Error> {
        todo!()
    }

    pub fn insert(&mut self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        assert!(self.id.is_none());

        let mut stmt = db.prepare_cached(
            "
            INSERT INTO todos
                (title, body, scheduled, deadline, state)
            VALUES
                (?, ?, ?, ?, ?)
            RETURNING id;
            ",
        )?;

        let id = stmt.query_row(
            rusqlite::params![
                self.title,
                self.body,
                self.scheduled.map(|d| d.to_string()),
                self.deadline.map(|d| d.to_string()),
                self.state
            ],
            |row| row.get(0),
        )?;

        self.id = id;

        Ok(())
    }

    pub fn update(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        let mut stmt = db.prepare_cached(
            "
            UPDATE todos SET
                title = ?,
                body = ?,
                scheduled = ?,
                deadline = ?,
                state = ?
            WHERE id = ?;
            ",
        )?;

        stmt.execute(rusqlite::params![
            self.title,
            self.body,
            self.scheduled.map(|d| d.to_string()),
            self.deadline.map(|d| d.to_string()),
            self.state,
            self.id
        ])?;
        Ok(())
    }

    pub fn upsert(&mut self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        match self.id {
            Some(_) => self.update(db),
            None => self.insert(db),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_todo_insert() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let mut todo = Todo::new("title".into(), "body".into(), None, None);
        todo.insert(&db).unwrap();
        assert!(todo.id.is_some());
    }

    #[test]
    fn test_todo_update() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let mut todo = Todo::new("title".into(), "body".into(), None, None);
        todo.insert(&db).unwrap();
        assert!(todo.id.is_some());
        todo.title = "some other title".into();
        todo.update(&db).unwrap();
    }
}
