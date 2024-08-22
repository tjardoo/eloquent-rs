use crate::{Columnable, QueryBuilder};

impl QueryBuilder {
    pub fn group_by<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.group_by.push(column.to_string());
        }

        self
    }
}
