use std::fmt::Display;

use error::EloquentError;

pub mod builder;
pub mod checker;
pub mod checks;
pub mod compiler;
pub mod error;

pub struct QueryBuilder {
    table: String,
    selects: Vec<Select>,
    conditions: Vec<Condition>,
    closures: Vec<(Logic, Vec<Condition>)>,
    joins: Vec<Join>,
    havings: Vec<Having>,
    group_by: Vec<String>,
    order_by: Vec<OrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub trait Columnable {
    fn to_columns(&self) -> Vec<String>;
}

struct Condition {
    field: String,
    operator: Operator,
    logic: Logic,
    values: Vec<Box<dyn ToSql>>,
}

#[derive(Debug, PartialEq)]
struct Select {
    column: String,
    function: Option<Function>,
    alias: Option<String>,
}

#[derive(Debug, PartialEq)]
struct OrderColumn {
    column: String,
    order: Order,
}

impl Select {
    fn format_column_name(&self) -> String {
        let column = match &self.function {
            Some(function) => match function {
                Function::Distinct => format!("{} {}", function, self.column),
                _ => format!("{}({})", function, self.column),
            },
            None => self.column.clone(),
        };

        if let Some(alias) = &self.alias {
            format!("{} AS {}", column, alias)
        } else {
            column
        }
    }

    fn format_column_name_without_alias(&self) -> String {
        match &self.function {
            Some(function) => match function {
                Function::Distinct => format!("{} {}", function, self.column),
                _ => format!("{}({})", function, self.column),
            },
            None => self.column.clone(),
        }
    }
}

struct Having {
    column: String,
    operator: Operator,
    value: i64,
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
enum Logic {
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Function {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    Distinct,
}

struct Join {
    table: String,
    left_hand: String,
    join_type: JoinType,
    right_hand: String,
}

enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Order {
    Asc,
    Desc,
}

pub trait PerformChecks {
    fn perform_checks(builder: &QueryBuilder) -> Result<(), EloquentError>;
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Operator::Equal => "=",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqual => "<=",
            Operator::Like => "LIKE",
            Operator::In => "IN",
            Operator::NotIn => "NOT IN",
            Operator::IsNull => "IS NULL",
            Operator::IsNotNull => "IS NOT NULL",
        };

        write!(f, "{}", operator)
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
        };

        write!(f, "{}", order)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let function = match self {
            Function::Count => "COUNT",
            Function::Sum => "SUM",
            Function::Avg => "AVG",
            Function::Min => "MIN",
            Function::Max => "MAX",
            Function::Distinct => "DISTINCT",
        };

        write!(f, "{}", function)
    }
}
