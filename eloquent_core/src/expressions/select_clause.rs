use crate::{Eloquent, error::EloquentError};
use super::formattable::Formattable;

pub struct SelectClauses {
    pub clauses: Vec<SelectClause>,
}

pub struct SelectClause {
    pub column: String,
}

impl Eloquent {
    /// Select clause
    ///
    /// It is used to retreive only those columns that are specified.
    /// If not specified it will retreive all columns.
    ///
    /// # Example
    ///
    /// This example will select the column `id` of all rows from the flights table.
    ///
    /// ```
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// let query = Eloquent::query()
    ///     .table("flights".to_string())
    ///     .select("id".to_string())
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn select(&mut self, column_name: String) -> &mut Eloquent {
        self.select_clauses.clauses.push(SelectClause {
            column: column_name,
        });

        self
    }
}

impl Formattable for SelectClauses {
    fn is_used(&self) -> bool {
        self.clauses.is_empty() == false
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        let mut query: String = "SELECT ".to_owned();

        if self.clauses.is_empty() {
            query.push_str("*");
        } else {
            let mut select_clauses = self.clauses.iter().peekable();

            while let Some(clause) = select_clauses.next() {
                let comma_or_empty;

                if select_clauses.peek().is_some() {
                    comma_or_empty = ", ";
                } else {
                    comma_or_empty = "";
                }

                let item = format!("`{}`{}",
                    clause.column,
                    comma_or_empty,
                );

                query.push_str(&item);
            }
        }

        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_single_select_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .select("first_name".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT `first_name` FROM users;");
    }

    #[test]
    fn it_can_create_multiple_select_queries() {
        let query = Eloquent::query()
            .table("users".to_string())
            .select("first_name".to_string())
            .select("last_name".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT `first_name`, `last_name` FROM users;");
    }

    #[test]
    fn it_selects_all_columns_if_no_select_query_set() {
        let query = Eloquent::query()
            .table("users".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users;");
    }
}
