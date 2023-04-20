use crate::{Eloquent, error::EloquentError, Clause};
use super::formattable::Formattable;

pub struct InsertClauses<'a> {
    pub table: Option<String>,
    pub clauses: Vec<Clause<'a>>,
}

impl<'a> Eloquent<'a> {
    /// Insert clause
    ///
    /// It is used to insert new records in the table.
    ///
    /// # Example
    ///
    /// This example will insert a new record flights table.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, GenericVar, Clause};
    ///
    /// let query = Eloquent::query()
    ///     .insert("flights", vec![
    ///         Clause {
    ///             column: "id".to_string(),
    ///             value: GenericVar::Int(1),
    ///         },
    ///         Clause {
    ///             column: "flight_code".to_string(),
    ///             value: GenericVar::Str("KL0803"),
    ///         },
    ///     ])
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn insert(&mut self, table_name: &str, clauses: Vec<Clause<'a>>) -> &mut Eloquent<'a> {
        self.insert_clause = InsertClauses {
            table: Some(table_name.to_string()),
            clauses,
        };

        self
    }
}

impl Formattable for InsertClauses<'_> {
    fn is_used(&self) -> bool {
        self.clauses.is_empty() == false
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        if self.clauses.is_empty() {
            return Ok("".to_string());
        }

        let mut columns: String = "".to_owned();
        let mut values: String = "".to_owned();

        let mut insert_clauses = self.clauses.iter().peekable();

        while let Some(clause) = insert_clauses.next() {
            let comma_or_empty;

            if insert_clauses.peek().is_some() {
                comma_or_empty = ", ";
            } else {
                comma_or_empty = "";
            }

            columns.push_str(&format!("`{}`{}",
                clause.column,
                comma_or_empty,
            ));

            values.push_str(&format!("{}{}",
                clause.value,
                comma_or_empty,
            ));
        }

        let table_name = match &self.table {
            Some(table_name) => Ok(table_name),
            None => Err(EloquentError::MissingTableNameError),
        };

        let query = format!("INSERT INTO {} ({}) VALUES ({})",
            table_name.unwrap(),
            columns,
            values
        );


        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::GenericVar;

    use super::*;

    #[test]
    fn it_can_create_a_single_insert_query() {
        let query = Eloquent::query()
            .insert("todos", vec![
                Clause {
                    column: "description".to_string(),
                    value: GenericVar::Str("learn Rust"),
                },
            ])
            .to_sql()
            .unwrap();

        assert_eq!(query, "INSERT INTO todos (`description`) VALUES (\"learn Rust\");");
    }

    #[test]
    fn it_can_create_a_multiple_insert_query() {
        let query = Eloquent::query()
            .insert("todos", vec![
                Clause {
                    column: "description".to_string(),
                    value: GenericVar::Str("learn Rust"),
                },
                Clause {
                    column: "is_completed".to_string(),
                    value: GenericVar::Bool(false),
                },
            ])
            .to_sql()
            .unwrap();

        assert_eq!(query, "INSERT INTO todos (`description`, `is_completed`) VALUES (\"learn Rust\", 0);");
    }
}
