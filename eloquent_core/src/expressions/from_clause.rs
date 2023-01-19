use crate::{Eloquent, error::EloquentError};

use super::formattable::Formattable;

pub struct FromClause {
    pub table: Option<String>,
}

impl Eloquent {
    pub fn table(&mut self, table_name: String) -> &mut Eloquent {
        self.from_clause = FromClause {
            table: Some(table_name),
        };

        self
    }
}

impl Formattable for FromClause {
    fn to_query_format(&self) -> Result<String, EloquentError> {
        match &self.table {
            Some(table_name) => Ok(format!("FROM {}", table_name)),
            None => Err(EloquentError::MissingTableNameError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, GenericVar};

    use super::*;

    #[test]
    fn it_can_set_the_table_name() {
        let query = Eloquent::query()
            .table("users".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users;");
    }

    #[test]
    fn it_throws_a_missing_table_name_error_if_no_table_name_set() {
        let query = Eloquent::query()
            .where_not("name".to_string(), GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap_err();

        assert_eq!(query, EloquentError::MissingTableNameError);
    }
}
