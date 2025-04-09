use crate::ID;
use crate::context::Context;
use crate::timestamp::TimeStamp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: ID<Todo>,
    pub title: String,
    pub path: String,
    pub scheduled: Option<TimeStamp>,
    pub deadline: Option<TimeStamp>,
    pub opened: TimeStamp,
    pub closed: Option<TimeStamp>,
}

fn path_from_title(title: &str) -> String {
    title
        .chars()
        .map(|c| if !c.is_alphanumeric() { '-' } else { c })
        .collect()
}

impl Todo {
    pub fn create(context: &Context, title: String) -> Result<Self, anyhow::Error> {
        let path = path_from_title(&title);
        let opened = TimeStamp::now();
        let mut stmt = context.db.prepare_cached(
            "
            INSERT INTO todos
                (title, path, opened)
            VALUES
                (?, ?, ?)
            RETURNING id;
            ",
        )?;

        let id: ID<Todo> = stmt.query_row(rusqlite::params![&title, &path, &opened], |row| {
            Ok(row.get(0)?)
        })?;

        Ok(Self {
            id,
            title,
            path,
            scheduled: None,
            deadline: None,
            opened,
            closed: None,
        })
    }

    pub fn fetch(context: &Context, id: ID<Todo>) -> Result<Self, anyhow::Error> {
        let mut stmt = context.db.prepare_cached(
            "
            SELECT 
                id, 
                title, 
                path,
                deadline,
                scheduled,
                opened,
                closed
            FROM todos
            WHERE id = ? LIMIT 1;
            ",
        )?;

        let todo: Self = stmt.query_row(rusqlite::params![id], |row| {
            Ok(Self {
                id: row.get(0)?,
                title: row.get(1)?,
                path: row.get(2)?,
                scheduled: row.get(3)?,
                deadline: row.get(4)?,
                opened: row.get(5)?,
                closed: row.get(6)?,
            })
        })?;

        Ok(todo)
    }

    pub fn fetch_all(context: &Context) -> Result<Vec<Self>, anyhow::Error> {
        context
            .db
            .prepare_cached(
                "
            SELECT
                id,
                title,
                path,
                scheduled,
                deadline,
                opened,
                closed
            FROM todos;
            ",
            )?
            .query(rusqlite::params![])?
            .and_then(|row| {
                Ok(Self {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    path: row.get(2)?,
                    scheduled: row.get(3)?,
                    deadline: row.get(4)?,
                    opened: row.get(5)?,
                    closed: row.get(6)?,
                })
            })
            .collect()
    }

    pub fn update(&self, context: &Context) -> Result<(), anyhow::Error> {
        let mut stmt = context.db.prepare_cached(
            "
            UPDATE todos SET
                title = ?,
                path = ?,
                scheduled = ?,
                deadline = ?,
                opened = ?,
                closed = ?
            WHERE id = ?;
            ",
        )?;

        let _ = stmt.execute(rusqlite::params![
            &self.title,
            &self.path,
            &self.scheduled,
            &self.deadline,
            &self.opened,
            &self.closed,
            &self.id
        ])?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prototodo_insert() {
        let context = Context::new_testing_context().unwrap();

        let _ = Todo::create(&context, "Hello World".into()).unwrap();
    }

    #[test]
    fn test_todo_fetch() {
        let context = Context::new_testing_context().unwrap();

        let expected = Todo::create(&context, "Hello World".into()).unwrap();
        let todo = Todo::fetch(&context, expected.id).unwrap();
        assert_eq!(expected, todo);
    }

    #[test]
    fn test_todo_fetchall() {
        let context = Context::new_testing_context().unwrap();

        let a = Todo::create(&context, "a".into()).unwrap();
        let b = Todo::create(&context, "b".into()).unwrap();
        let c = Todo::create(&context, "c".into()).unwrap();

        let expected = vec![a, b, c];
        let mut got = Todo::fetch_all(&context).unwrap();

        got.sort_by(|a, b| a.title.cmp(&b.title));

        dbg!(&expected);
        dbg!(&got);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_todo_update() {
        let context = Context::new_testing_context().unwrap();
        let mut expected = Todo::create(&context, "todo".into()).unwrap();

        expected.title = "Hello World".into();

        expected.deadline = Some(TimeStamp::now());
        expected.scheduled = Some(TimeStamp::now());
        expected.closed = Some(TimeStamp::now());

        expected.update(&context).unwrap();
        let got = Todo::fetch(&context, expected.id).unwrap();

        dbg!(&expected);
        dbg!(&got);

        assert_eq!(expected, got);
    }
}
