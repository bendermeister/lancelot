use crate::todo::Todo;
use std::collections::HashMap;

pub trait Storer {
    fn store(&mut self, todo: &Todo) -> Result<(), anyhow::Error>;
    fn read<'a>(&'a self, todo: &Todo) -> Result<Option<&'a str>, anyhow::Error>;
    fn has_changed(&self, todo: &Todo) -> Result<bool, anyhow::Error>;
}

pub struct TestStore {
    map: HashMap<String, String>,
}

impl Storer for TestStore {
    fn store(&mut self, todo: &Todo) -> Result<(), anyhow::Error> {
        self.map.insert(todo.path.clone(), "".into());
        Ok(())
    }

    fn read<'a>(&'a self, todo: &Todo) -> Result<Option<&'a str>, anyhow::Error> {
        Ok(self.map.get(&todo.path).map(|d| d.as_str()))
    }

    fn has_changed(&self, _: &Todo) -> Result<bool, anyhow::Error> {
        return Ok(false);
    }
}
