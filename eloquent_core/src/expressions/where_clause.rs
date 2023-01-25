use std::fmt;

use crate::{Eloquent, error::EloquentError, GenericVar};
use super::formattable::Formattable;

pub struct WhereClauses {
    pub clauses: Vec<WhereClause>,
}

pub struct WhereClause{
    pub column: String,
    pub value: GenericVar,
    pub operator: WhereOperator,
}

pub enum WhereOperator {
    Where,
    WhereNot,
}

impl Eloquent {
    /// Where clause
    ///
    /// It is used to extract only those records that fulfill the specified condition.
    ///
    /// # Example
    ///
    /// ```rs
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// This example will select all records from the flights table where the `destination` is `SIN` (Singapore).
    ///
    /// let query = Eloquent::query()
    ///     .table("flights".to_string())
    ///     .r#where("destination".to_string(), GenericVar::Str("SIN".to_string()))
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn r#where(&mut self, column_name: String, value: GenericVar) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name,
            value,
            operator: WhereOperator::Where,
        });

        self
    }

    /// Where Not clause
    ///
    /// It is used to extract only those records that do NOT fulfill the specified condition.
    ///
    /// # Example
    ///
    /// This example will select all records from the flights table where the `destination` is NOT `SIN` (Singapore).
    ///
    /// ```rs
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// let query = Eloquent::query()
    ///     .table("flights".to_string())
    ///     .r#where("destination".to_string(), GenericVar::Str("SIN".to_string()))
    ///     .to_sql()
    ///     .unwrap();
    /// ```
    pub fn where_not(&mut self, column_name: String, value: GenericVar) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name,
            value,
            operator: WhereOperator::WhereNot,
        });

        self
    }
}

impl Formattable for WhereClauses {
    fn is_used(&self) -> bool {
        self.clauses.is_empty() == false
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        if self.clauses.is_empty() {
            return Ok("".to_string());
        }

        let mut query: String = " WHERE ".to_owned();

        let mut where_clauses = self.clauses.iter().peekable();

        while let Some(clause) = where_clauses.next() {
            let and_or_empty;

            if where_clauses.peek().is_some() {
                and_or_empty = " AND ";
            } else {
                and_or_empty = "";
            }

            let item = format!("`{}` {} {}{}",
                clause.column,
                clause.operator,
                clause.value,
                and_or_empty,
            );

            query.push_str(&item);
        }

        Ok(query)
    }
}

impl fmt::Display for WhereOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WhereOperator::Where => write!(f, "="),
            WhereOperator::WhereNot => write!(f, "!="),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_single_where_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("name".to_string(), GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` = \"John\";");
    }

    #[test]
    fn it_can_create_a_single_where_not_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .where_not("name".to_string(), GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` != \"John\";");
    }

    #[test]
    fn it_can_create_multiple_where_queries() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("first_name".to_string(), GenericVar::Str("John".to_string()))
            .r#where("last_name".to_string(), GenericVar::Str("Doe".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `first_name` = \"John\" AND `last_name` = \"Doe\";");
    }

    #[test]
    fn it_can_create_multiple_where_not_queries() {
        let query = Eloquent::query()
            .table("users".to_string())
            .where_not("first_name".to_string(), GenericVar::Str("John".to_string()))
            .where_not("last_name".to_string(), GenericVar::Str("Doe".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `first_name` != \"John\" AND `last_name` != \"Doe\";");
    }

    #[test]
    fn it_can_create_query_without_where_clauses() {
        let query = Eloquent::query()
            .table("users".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users;");
    }

    #[test]
    fn it_can_create_a_where_query_with_string_value() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("name".to_string(), GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` = \"John\";");
    }

    #[test]
    fn it_can_create_a_where_query_with_integer_value() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("age".to_string(), GenericVar::Int(25))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `age` = 25;");
    }

    #[test]
    fn it_can_create_a_where_query_with_boolean_value() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("is_active".to_string(), GenericVar::Bool(true))
            .r#where("is_blocked".to_string(), GenericVar::Bool(false))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `is_active` = 1 AND `is_blocked` = 0;");
    }
}
