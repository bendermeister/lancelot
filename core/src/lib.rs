use chrono::NaiveDate;
use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, PartialEq, Eq)]
pub struct ID<T> {
    id: i64,
    marker: PhantomData<T>,
}

impl<T> Clone for ID<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            marker: PhantomData
        }
    }
}

impl<T> Copy for ID<T> {
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

impl FromSql for TodoState {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let state = <i64 as FromSql>::column_result(value)?;
        match state {
            0 => Ok(TodoState::Todo),
            1 => Ok(TodoState::Done),
            e => Err(rusqlite::types::FromSqlError::OutOfRange(e)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProtoTodo {
    pub title: String,
    pub body: String,
    pub scheduled: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub state: TodoState,
}

impl From<Todo> for ProtoTodo {
    fn from(value: Todo) -> Self {
        Self {
            title: value.title,
            body: value.body,
            scheduled: value.scheduled,
            deadline: value.deadline,
            state: value.state,
        }
    }
}

impl ProtoTodo {
    pub fn new(
        title: String,
        body: String,
        scheduled: Option<NaiveDate>,
        deadline: Option<NaiveDate>,
        state: TodoState,
    ) -> Self {
        Self {
            title,
            body,
            scheduled,
            deadline,
            state,
        }
    }

    pub fn insert(self, db: &rusqlite::Connection) -> Result<Todo, anyhow::Error> {
        let mut stmt = db.prepare_cached(
            "
            INSERT INTO todos
                (title, body, scheduled, deadline, state)
            VALUES
                (?, ?, ?, ?, ?)
            RETURNING id;
            ",
        )?;

        let id: ID<Todo> = stmt.query_row(
            rusqlite::params![
                &self.title,
                &self.body,
                &self.scheduled.map(|d| d.to_string()),
                &self.deadline.map(|d| d.to_string()),
                self.state
            ],
            |row| row.get(0),
        )?;

        Ok(Todo {
            id,
            title: self.title,
            body: self.body,
            scheduled: self.scheduled,
            deadline: self.deadline,
            state: self.state,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: ID<Todo>,
    pub title: String,
    pub body: String,
    pub scheduled: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub state: TodoState,
}

impl Todo {
    pub fn fetch(db: &rusqlite::Connection, id: ID<Self>) -> Result<Self, anyhow::Error> {
        let mut stmt = db.prepare_cached(
            "
            SELECT
                id,
                title,
                body,
                scheduled,
                deadline,
                state
            FROM todos WHERE 
                id = ? LIMIT 1;
            ",
        )?;

        let parse_date = |s: Option<String>| match s.map(|d| d.parse::<NaiveDate>()) {
            Some(Ok(d)) => Ok(Some(d)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        };

        let todo = stmt.query_row(rusqlite::params![id], |row| {
            let scheduled = match parse_date(row.get(3)?) {
                Ok(d) => d,
                Err(e) => {
                    return Err(rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        e.into(),
                    ));
                }
            };

            let deadline = match parse_date(row.get(4)?) {
                Ok(d) => d,
                Err(e) => {
                    return Err(rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        e.into(),
                    ));
                }
            };


            Ok(Self {
                id: row.get(0)?,
                title: row.get(1)?,
                body: row.get(2)?,
                scheduled,
                deadline,
                state: row.get(5)?,
            })
        })?;

        Ok(todo)
    }

    pub fn fetch_all(db: &rusqlite::Connection) -> Result<Vec<Self>, anyhow::Error> {
        let mut stmt = db.prepare_cached(
            "
            SELECT 
                id, 
                title, 
                body, 
                scheduled, 
                deadline, 
                state 
            FROM todos;
            ",
        )?;

        let parse_date = |s: Option<String>| match s.map(|d| d.parse::<NaiveDate>()) {
            Some(Ok(s)) => Ok(Some(s)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        };

        stmt.query(rusqlite::params![])?
            .and_then(|row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    body: row.get(2)?,
                    scheduled: parse_date(row.get(3)?)?,
                    deadline: parse_date(row.get(4)?)?,
                    state: row.get(5)?,
                })
            })
            .collect()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_todo_insert() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let todo = ProtoTodo::new("title".into(), "body".into(), None, None, TodoState::Todo);
        let out = todo.clone().insert(&db).unwrap();
        assert_eq!(todo, out.clone().into());

        let todos = Todo::fetch_all(&db).unwrap();
        assert_eq!(1, todos.len());
        assert_eq!(out, todos[0]);
    }

    #[test]
    fn test_todo_update() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let todo = ProtoTodo::new("title".into(), "body".into(), None, None, TodoState::Todo);
        let mut todo = todo.insert(&db).unwrap();
        todo.title = "some other title".into();
        todo.update(&db).unwrap();

        let todos = Todo::fetch_all(&db).unwrap();
        assert_eq!(1, todos.len());

        assert_eq!(todo, todos[0]);
    }

    #[test]
    fn test_todo_fetch() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        mittelmeer::migrate(&db).unwrap();

        let todo = ProtoTodo::new("title".into(), "body".into(), None, None, TodoState::Todo);
        let expected = todo.insert(&db).unwrap();
        let todo = Todo::fetch(&db, expected.id).unwrap();
        assert_eq!(expected, todo);
    }
}
