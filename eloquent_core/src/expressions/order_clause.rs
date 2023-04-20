use std::fmt;
use crate::{Eloquent, error::EloquentError, Direction};

use super::formattable::Formattable;

pub struct OrderClauses {
    pub clauses: Vec<OrderClause>,
}

pub struct OrderClause {
    pub column: String,
    pub direction: Direction,
}

impl<'a> Eloquent<'a> {
    pub fn order_by(&mut self, column_name: &str, direction: Direction) -> &mut Eloquent<'a> {
        self.order_clauses.clauses.push(OrderClause {
            column: column_name.to_string(),
            direction,
        });

        self
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Asc => write!(f, "ASC"),
            Direction::Desc => write!(f, "DESC"),
        }
    }
}

impl Formattable for OrderClauses {
    fn is_used(&self) -> bool {
        self.clauses.is_empty() == false
    }

    fn to_query_format(&self) -> Result<String, EloquentError> {
        if self.clauses.is_empty() {
            return Ok("".to_string());
        }

        let mut query: String = " ORDER BY ".to_owned();

        let mut order_clauses = self.clauses.iter().peekable();

        while let Some(clause) = order_clauses.next() {
            let comma_or_empty;

            if order_clauses.peek().is_some() {
                comma_or_empty = ", ";
            } else {
                comma_or_empty = "";
            }

            let item = format!("`{}` {}{}",
                clause.column,
                clause.direction,
                comma_or_empty,
            );

            query.push_str(&item);
        }

        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_order_by_a_single_column_asc() {
        let query = Eloquent::query()
            .table("users")
            .order_by("id", Direction::Asc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users ORDER BY `id` ASC;");
    }

    #[test]
    fn it_can_order_by_a_single_column_desc() {
        let query = Eloquent::query()
            .table("users")
            .order_by("id", Direction::Desc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users ORDER BY `id` DESC;");
    }

    #[test]
    fn it_can_order_by_multiple_columns() {
        let query = Eloquent::query()
            .table("flights")
            .order_by("destination", Direction::Asc)
            .order_by("terminal_id", Direction::Desc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM flights ORDER BY `destination` ASC, `terminal_id` DESC;");
    }
}
