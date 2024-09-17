//! # Eloquent Core
//!
//! The core library for building SQL queries. This library is used by the [Eloquent](https://crates.io/crates/eloquent) library to build SQL queries.

use compiler::{build_statement, build_substatement};
use error::EloquentError;
use std::fmt::Display;

mod builders;
mod checks;
mod compiler;
mod compilers;
/// The error module that contains all the possible errors that can occur while building a query.
pub mod error;
mod queries;
mod query_builder;
mod subqueries;
mod subquery_builder;
mod validator;

/// The main builder struct that holds all the query building information.
pub struct QueryBuilder {
    table: Option<String>,
    selects: Vec<Select>,
    inserts: Vec<Insert>,
    updates: Vec<Update>,
    delete: bool,
    conditions: Vec<Condition>,
    closures: Vec<(Logic, Vec<Condition>)>,
    joins: Vec<Join>,
    havings: Vec<Having>,
    group_by: Vec<String>,
    order_by: Vec<OrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
    enable_checks: bool,
}

/// The subquery builder struct that holds all the subquery building information.
pub struct SubqueryBuilder {
    table: Option<String>,
    selects: Vec<Select>,
    conditions: Vec<Condition>,
    joins: Vec<Join>,
    havings: Vec<Having>,
    group_by: Vec<String>,
    order_by: Vec<OrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}

pub trait ToSql {
    fn to_sql(&self) -> Result<String, EloquentError>;

    fn is_subquery(&self) -> bool {
        false
    }
}

pub trait Columnable {
    fn to_columns(&self) -> Vec<String>;
}

pub trait Selectable {
    fn to_select_column(&self) -> String;
}

pub(crate) trait PerformChecks {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError>;
}

#[allow(clippy::borrowed_box)]
pub(crate) trait SqlBuilder {
    fn build<'a>(
        builder: &'a QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
    ) -> Result<String, EloquentError>;
}

pub(crate) enum Action {
    Select,
    Insert,
    Update,
    Delete,
}

struct Condition {
    field: String,
    operator: Operator,
    logic: Logic,
    values: Vec<Box<dyn ToSql>>,
}

struct Select {
    column: String,
    function: Option<Function>,
    alias: Option<String>,
}

struct Insert {
    column: String,
    value: Box<dyn ToSql>,
}

struct Update {
    column: String,
    value: Box<dyn ToSql>,
}

#[derive(PartialEq)]
struct OrderColumn {
    column: String,
    order: Order,
}

struct Having {
    conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Between,
    Like,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Date,
    Year,
    Month,
    Day,
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

impl Selectable for &str {
    fn to_select_column(&self) -> String {
        self.to_string()
    }
}

impl Selectable for SubqueryBuilder {
    fn to_select_column(&self) -> String {
        self.to_sql().unwrap()
    }
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
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(format!("'{}'", self.replace('\'', "''")))
    }
}

impl ToSql for String {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(format!("'{}'", self.replace('\'', "''")))
    }
}

impl ToSql for &String {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(format!("'{}'", self.replace('\'', "''")))
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_string())
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_string())
    }
}

impl ToSql for f32 {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_string())
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_string())
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_string())
    }
}

impl ToSql for QueryBuilder {
    fn to_sql(&self) -> Result<String, EloquentError> {
        build_statement(self)
    }
}

impl ToSql for SubqueryBuilder {
    fn to_sql(&self) -> Result<String, EloquentError> {
        build_substatement(self)
    }

    fn is_subquery(&self) -> bool {
        true
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
            Operator::Between => "BETWEEN",
            Operator::Like => "LIKE",
            Operator::In => "IN",
            Operator::NotIn => "NOT IN",
            Operator::IsNull => "IS NULL",
            Operator::IsNotNull => "IS NOT NULL",
            Operator::Date => "DATE",
            Operator::Year => "YEAR",
            Operator::Month => "MONTH",
            Operator::Day => "DAY",
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

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let join_type = match self {
            JoinType::Inner => "JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
            JoinType::Full => "FULL JOIN",
        };

        write!(f, "{}", join_type)
    }
}

impl Condition {
    fn format_sql(&self) -> String {
        let values = self
            .values
            .iter()
            .map(|v| v.to_sql().unwrap())
            .collect::<Vec<String>>()
            .join(", ");

        match self.operator {
            Operator::Between => format!(
                "{} {} {} AND {}",
                self.field,
                self.operator,
                values.split(", ").next().unwrap(),
                values.split(", ").last().unwrap()
            ),
            Operator::In | Operator::NotIn => {
                if self.values.iter().any(|v| v.is_subquery()) {
                    // subquery already contains parentheses so we don't need to add them
                    format!("{} {} {}", self.field, self.operator, values)
                } else {
                    format!("{} {} ({})", self.field, self.operator, values)
                }
            }
            Operator::IsNull | Operator::IsNotNull => format!("{} {}", self.field, self.operator),
            Operator::Date | Operator::Year | Operator::Month | Operator::Day => {
                format!("{}({}) = {}", self.operator, self.field, values)
            }
            _ => format!("{} {} {}", self.field, self.operator, values),
        }
    }
}
