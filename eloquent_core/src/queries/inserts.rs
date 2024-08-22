use crate::{Insert, QueryBuilder, ToSql};

impl QueryBuilder {
    pub fn insert(mut self, column: &str, value: impl ToSql + 'static) -> Self {
        self.inserts.push(Insert {
            column: column.to_string(),
            value: Box::new(value),
        });

        self
    }
}
