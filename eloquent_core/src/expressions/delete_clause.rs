use crate::{Eloquent, error::EloquentError};
use super::formattable::Formattable;

pub struct DeleteClause {
    pub table: Option<String>,
}

impl Eloquent {
    /// Delete clause
    ///
    /// It is used to delete existing records in the table.
    ///
    /// # Example
    ///
    /// This example will delete the record with `id` is `1` in the flights table.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// let query = Eloquent::query()
    ///     .delete("flights".to_string())
    ///     .r#where("id".to_string(), GenericVar::Int(1))
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn delete(&mut self, table_name: String) -> &mut Eloquent {
        self.delete_clause = DeleteClause {
            table: Some(table_name),
        };

        self
    }
}

impl Formattable for DeleteClause {
    fn is_used(&self) -> bool {
        self.table.is_some()
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        match &self.table {
            Some(table_name) => Ok(format!("DELETE FROM {}", table_name)),
            None => Err(EloquentError::MissingTableNameError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GenericVar;

    use super::*;

    #[test]
    fn it_can_create_a_delete_query() {
        let query = Eloquent::query()
            .delete("todos".to_string())
            .r#where("id".to_string(), GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "DELETE FROM todos WHERE `id` = 1;");
    }
}
