use chrono::NaiveDate;
use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ID<T> {
    id: usize,
    marker: PhantomData<T>,
}

impl<T> ID<T> {
    pub fn new(id: usize) -> Self {
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
            marker: PhantomData
        }
    }
}

impl<T> Deref for ID<T> {
    type Target = usize;

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
        let id: usize = <usize as FromSql>::column_result(value)?;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub id: ID<Todo>,
    pub scheduled: NaiveDate,
    pub deadline: NaiveDate,
    pub state: TodoState,
}
