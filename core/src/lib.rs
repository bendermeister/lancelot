use rusqlite::types::{FromSql, ToSql, ToSqlOutput};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

pub mod config;
pub mod context;
pub mod db_wrapper;
pub mod store;
pub mod timestamp;
pub mod todo;

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

