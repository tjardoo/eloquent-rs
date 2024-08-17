pub mod builder;
pub mod compiler;

// export query builder
pub use builder::QueryBuilder;

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub trait Columnable {
    fn to_columns(&self) -> Vec<String>;
}

pub struct Condition {
    field: String,
    operator: Operator,
    logic: Logic,
    values: Vec<Box<dyn ToSql>>,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

#[derive(Debug, PartialEq)]
pub enum Logic {
    And,
    Or,
}

impl Condition {
    fn new(field: &str, operator: Operator, logic: Logic, values: Vec<Box<dyn ToSql>>) -> Self {
        Condition {
            field: field.to_string(),
            operator,
            logic,
            values,
        }
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self)
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl Columnable for &str {
    fn to_columns(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl Columnable for Vec<&str> {
    fn to_columns(&self) -> Vec<String> {
        self.iter().map(|&s| s.to_string()).collect()
    }
}
