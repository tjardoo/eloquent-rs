use std::fmt;
use crate::{Eloquent, error::EloquentError};
use super::formattable::Formattable;

pub struct WhereClauses {
    pub clauses: Vec<WhereClause>,
}

pub struct WhereClause {
    pub column: String,
    pub value: String,
    pub operator: WhereOperator,
}

pub enum WhereOperator {
    Where,
    WhereNot,
}

impl Eloquent {
    pub fn r#where(&mut self, column_name: String, value: String) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name,
            value,
            operator: WhereOperator::Where,
        });

        self
    }

    pub fn where_not(&mut self, column_name: String, value: String) -> &mut Eloquent {
        self.where_clauses.clauses.push(WhereClause {
            column: column_name,
            value,
            operator: WhereOperator::WhereNot,
        });

        self
    }
}

impl Formattable for WhereClauses {
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

            let item = format!("{} {} \"{}\"{}",
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
            .r#where("name".to_string(), "John".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE name = \"John\";");
    }

    #[test]
    fn it_can_create_a_single_where_not_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .where_not("name".to_string(), "John".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE name != \"John\";");
    }

    #[test]
    fn it_can_create_multiple_where_queries() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("first_name".to_string(), "John".to_string())
            .r#where("last_name".to_string(), "Doe".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE first_name = \"John\" AND last_name = \"Doe\";");
    }

    #[test]
    fn it_can_create_multiple_where_not_queries() {
        let query = Eloquent::query()
            .table("users".to_string())
            .where_not("first_name".to_string(), "John".to_string())
            .where_not("last_name".to_string(), "Doe".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE first_name != \"John\" AND last_name != \"Doe\";");
    }

    #[test]
    fn it_can_create_query_without_where_clauses() {
        let query = Eloquent::query()
            .table("users".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users;");
    }
}
