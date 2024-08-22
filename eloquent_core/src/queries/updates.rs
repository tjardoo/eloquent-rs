use crate::{QueryBuilder, ToSql, Update};

impl QueryBuilder {
    pub fn update(mut self, column: &str, value: impl ToSql + 'static) -> Self {
        self.updates.push(Update {
            column: column.to_string(),
            value: Box::new(value),
        });

        self
    }
}
