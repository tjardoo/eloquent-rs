use crate::{Eloquent, error::EloquentError, Clause};
use super::formattable::Formattable;

pub struct UpdateClauses {
    pub table: Option<String>,
    pub clauses: Vec<Clause>,
}

impl Eloquent {
    /// Update clause
    ///
    /// It is used to update existing records in the table.
    ///
    /// # Example
    ///
    /// This example will update the `flight_code` column with KL0803 where the `id` is `1` in the flights table.
    ///
    /// ```rs
    /// use eloquent_core::{Eloquent, GenericVar, Clause};
    ///
    /// let query = Eloquent::query()
    ///     .update("flights".to_string(), vec![
    ///         Clause {
    ///             column: "flight_code".to_string(),
    ///             value: GenericVar::Str("KL0803".to_string()),
    ///         },
    ///     ])
    ///     .r#where("id".to_string(), GenericVar::Int(1))
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn update(&mut self, table_name: String, clauses: Vec<Clause>) -> &mut Eloquent {
        self.update_clause = UpdateClauses {
            table: Some(table_name),
            clauses,
        };

        self
    }
}

impl Formattable for UpdateClauses {
    fn is_used(&self) -> bool {
        self.clauses.is_empty() == false
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        if self.clauses.is_empty() {
            return Ok("".to_string());
        }

        let table_name = match &self.table {
            Some(table_name) => Ok(table_name),
            None => Err(EloquentError::MissingTableNameError),
        };

        let mut query: String = format!("UPDATE {} SET ", table_name.unwrap());

        let mut update_clauses = self.clauses.iter().peekable();

        while let Some(clause) = update_clauses.next() {
            let comma_or_empty;

            if update_clauses.peek().is_some() {
                comma_or_empty = ", ";
            } else {
                comma_or_empty = "";
            }

            let item = format!("`{}` = {}{}",
                clause.column,
                clause.value,
                comma_or_empty,
            );

            query.push_str(&item);
        }


        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::GenericVar;

    use super::*;

    #[test]
    fn it_can_create_a_single_update_query() {
        let query = Eloquent::query()
            .update("todos".to_string(), vec![
                Clause {
                    column: "description".to_string(),
                    value: GenericVar::Str("learn Rust".to_string()),
                },
            ])
            .r#where("id".to_string(), GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "UPDATE todos SET `description` = \"learn Rust\" WHERE `id` = 1;");
    }

    #[test]
    fn it_can_create_a_multiple_update_query() {
        let query = Eloquent::query()
            .update("todos".to_string(), vec![
                Clause {
                    column: "description".to_string(),
                    value: GenericVar::Str("learn Rust".to_string()),
                },
                Clause {
                    column: "is_completed".to_string(),
                    value: GenericVar::Bool(false),
                },
            ])
            .r#where("id".to_string(), GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "UPDATE todos SET `description` = \"learn Rust\", `is_completed` = 0 WHERE `id` = 1;");
    }
}
