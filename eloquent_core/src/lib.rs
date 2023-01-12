use std::fmt;

mod error;

pub struct Eloquent {
    pub table: Option<String>,
    pub clauses: Vec<Clause>,
}

pub struct Clause {
    pub column: String,
    pub value: String,
    pub operator: Operator,
}

pub enum Operator {
    Where,
    WhereNot,
}

impl Eloquent {
    pub fn query() -> Eloquent {
        Eloquent {
            table: None,
            clauses: vec![],
        }
    }

    pub fn table(&mut self, table_name: String) -> &mut Eloquent {
        self.table = Some(table_name);

        self
    }

    pub fn r#where(&mut self, column_name: String, value: String) -> &mut Eloquent {
        self.clauses.push(Clause {
            column: column_name,
            value,
            operator: Operator::Where,
        });

        self
    }

    pub fn where_not(&mut self, column_name: String, value: String) -> &mut Eloquent {
        self.clauses.push(Clause {
            column: column_name,
            value,
            operator: Operator::WhereNot,
        });

        self
    }

    pub fn to_sql(&mut self) -> Result<String, error::EloquentError> {
        let table_name = if let Some(table_name) = &self.table {
            table_name
        } else {
            return Err(error::EloquentError::MissingTableNameError);
        };

        Ok(format!("SELECT * FROM {} WHERE {} {} \"{}\";",
            table_name,
            self.clauses[0].column,
            self.clauses[0].operator,
            self.clauses[0].value,
        ))
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Where => write!(f, "="),
            Operator::WhereNot => write!(f, "!="),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::EloquentError;

    use super::*;

    #[test]
    fn it_can_create_a_where_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .r#where("name".to_string(), "John".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE name = \"John\";");
    }

    #[test]
    fn it_can_create_a_where_not_query() {
        let query = Eloquent::query()
            .table("users".to_string())
            .where_not("name".to_string(), "John".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM users WHERE name != \"John\";");
    }

    #[test]
    fn it_throws_a_missing_table_name_error_if_no_table_name_set() {
        let query = Eloquent::query()
            .where_not("name".to_string(), "John".to_string())
            .to_sql()
            .unwrap_err();

        assert_eq!(query, EloquentError::MissingTableNameError);
    }
}
