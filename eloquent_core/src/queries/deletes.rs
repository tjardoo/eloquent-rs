use crate::QueryBuilder;

impl QueryBuilder {
    pub fn delete(mut self) -> Self {
        self.delete = true;

        self
    }
}
