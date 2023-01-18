use crate::{Eloquent, error::EloquentError};

use super::formattable::Formattable;

pub struct GroupClauses {
    pub clauses: Vec<GroupClause>,
}

pub struct GroupClause {
    pub column: String,
}

impl Eloquent {
    pub fn group_by(&mut self, column_name: String) -> &mut Eloquent {
        self.group_clauses.clauses.push(GroupClause {
            column: column_name,
        });

        self
    }
}

impl Formattable for GroupClauses {
    fn to_query_format(&self) -> Result<String, EloquentError> {
        if self.clauses.is_empty() {
            return Ok("".to_string());
        }

        let mut query: String = " GROUP BY ".to_owned();

        let mut order_clauses = self.clauses.iter().peekable();

        while let Some(clause) = order_clauses.next() {
            let comma_or_empty;

            if order_clauses.peek().is_some() {
                comma_or_empty = ", ";
            } else {
                comma_or_empty = "";
            }

            let item = format!("{}{}",
                clause.column,
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
    fn it_can_group_by_a_single_column_asc() {
        let query = Eloquent::query()
            .table("users".to_string())
            .group_by("country_id".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users GROUP BY country_id;");
    }

    #[test]
    fn it_can_group_by_multiple_columns() {
        let query = Eloquent::query()
            .table("users".to_string())
            .group_by("country_id".to_string())
            .group_by("city".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users GROUP BY country_id, city;");
    }
}
