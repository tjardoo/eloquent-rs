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
    WhereNull,
    WhereNotNull,
}

impl Eloquent {
    /// Where clause
    ///
    /// It is used to extract only those records that fulfill the specified condition.
    ///
    /// # Example
    ///
    /// This example will select all records from the flights table where the `destination` is `SIN` (Singapore).
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// let query = Eloquent::query()
    ///     .table("flights")
    ///     .r#where("destination", GenericVar::Str("SIN".to_string()))
    ///     .to_sql()
    ///     .unwrap();
    ///
    /// assert_eq!(query, "SELECT * FROM flights WHERE `destination` = \"SIN\";");
    /// ```
    pub fn r#where(&mut self, column_name: &str, value: GenericVar) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name.to_string(),
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
    /// ```rust
    /// use eloquent_core::{Eloquent, GenericVar};
    ///
    /// let query = Eloquent::query()
    ///     .table("flights")
    ///     .where_not("destination", GenericVar::Str("SIN".to_string()))
    ///     .to_sql()
    ///     .unwrap();
    ///
    /// assert_eq!(query, "SELECT * FROM flights WHERE `destination` != \"SIN\";");
    /// ```
    pub fn where_not(&mut self, column_name: &str, value: GenericVar) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name.to_string(),
            value,
            operator: WhereOperator::WhereNot,
        });

        self
    }

    /// Where Null clause
    ///
    /// It is used to extract only those records that do NOT fulfill the specified condition.
    ///
    /// # Example
    ///
    /// This example will select all records from the flights table where the `destination` is NULL.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let query = Eloquent::query()
    ///     .table("flights")
    ///     .where_null("destination")
    ///     .to_sql()
    ///     .unwrap();
    ///
    /// assert_eq!(query, "SELECT * FROM flights WHERE `destination` IS NULL;");
    /// ```
    pub fn where_null(&mut self, column_name: &str) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name.to_string(),
            value: GenericVar::None,
            operator: WhereOperator::WhereNull,
        });

        self
    }

    /// Where Not Null clause
    ///
    /// It is used to extract only those records that do NOT fulfill the specified condition.
    ///
    /// # Example
    ///
    /// This example will select all records from the flights table where the `destination` is NOT NULL.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let query = Eloquent::query()
    ///     .table("flights")
    ///     .where_not_null("destination")
    ///     .to_sql()
    ///     .unwrap();
    ///
    /// assert_eq!(query, "SELECT * FROM flights WHERE `destination` IS NOT NULL;");
    /// ```
    pub fn where_not_null(&mut self, column_name: &str) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name.to_string(),
            value: GenericVar::None,
            operator: WhereOperator::WhereNotNull,
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

            let item = format!("`{}` {}{}{}",
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
            WhereOperator::Where => write!(f, "= "),
            WhereOperator::WhereNot => write!(f, "!= "),
            WhereOperator::WhereNull => write!(f, "IS NULL"),
            WhereOperator::WhereNotNull => write!(f, "IS NOT NULL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_single_where_query() {
        let query = Eloquent::query()
            .table("users")
            .r#where("name", GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` = \"John\";");
    }

    #[test]
    fn it_can_create_a_single_where_not_query() {
        let query = Eloquent::query()
            .table("users")
            .where_not("name", GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` != \"John\";");
    }

    #[test]
    fn it_can_create_multiple_where_queries() {
        let query = Eloquent::query()
            .table("users")
            .r#where("first_name", GenericVar::Str("John".to_string()))
            .r#where("last_name", GenericVar::Str("Doe".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `first_name` = \"John\" AND `last_name` = \"Doe\";");
    }

    #[test]
    fn it_can_create_multiple_where_not_queries() {
        let query = Eloquent::query()
            .table("users")
            .where_not("first_name", GenericVar::Str("John".to_string()))
            .where_not("last_name", GenericVar::Str("Doe".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `first_name` != \"John\" AND `last_name` != \"Doe\";");
    }

    #[test]
    fn it_can_create_query_without_where_clauses() {
        let query = Eloquent::query()
            .table("users")
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users;");
    }

    #[test]
    fn it_can_create_a_where_query_with_string_value() {
        let query = Eloquent::query()
            .table("users")
            .r#where("name", GenericVar::Str("John".to_string()))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `name` = \"John\";");
    }

    #[test]
    fn it_can_create_a_where_query_with_integer_value() {
        let query = Eloquent::query()
            .table("users")
            .r#where("age", GenericVar::Int(25))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `age` = 25;");
    }

    #[test]
    fn it_can_create_a_where_query_with_boolean_value() {
        let query = Eloquent::query()
            .table("users")
            .r#where("is_active", GenericVar::Bool(true))
            .r#where("is_blocked", GenericVar::Bool(false))
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `is_active` = 1 AND `is_blocked` = 0;");
    }

    #[test]
    fn it_can_create_a_single_where_null_query() {
        let query = Eloquent::query()
            .table("users")
            .where_null("location")
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `location` IS NULL;");
    }

    #[test]
    fn it_can_create_a_single_where_not_null_query() {
        let query = Eloquent::query()
            .table("users")
            .where_not_null("location")
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE `location` IS NOT NULL;");
    }
}
