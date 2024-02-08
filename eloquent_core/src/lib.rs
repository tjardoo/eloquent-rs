//! # Eloquent Core
//!
//! `eloquent_core` is a library for building SQL queries in Rust.
//!

use std::fmt::Display;

use builder::Bindings;

mod builder;
mod compiler;

pub struct Eloquent {
    bindings: Bindings,
}

impl Eloquent {
    pub fn query() -> Self {
        Self {
            bindings: Bindings {
                select: vec![],
                insert: vec![],
                update: vec![],
                table: None,
                join: vec![],
                r#where: vec![],
                where_closure: vec![],
                group_by: vec![],
                having: vec![],
                order_by: vec![],
                is_delete: false,
                limit: None,
                offset: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Like,
    NotLike,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    String(String),
    Int(u32),
    Bool(bool),
    Null,
}

pub enum Direction {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhereOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Count,
    Max,
    Min,
    Sum,
    Avg,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub column: String,
    pub operator: Operator,
    pub value: Variable,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: Operator,
    pub value: Variable,
    pub where_operator: WhereOperator,
}

#[derive(Debug, Clone)]
pub struct Join {
    pub table: String,
    pub left_hand: String,
    pub right_hand: String,
    pub r#type: JoinType,
}

#[derive(Debug, Clone)]
pub struct WhereClosure {
    pub closure: Vec<Clause>,
    pub where_operator: WhereOperator,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::Like => write!(f, "LIKE"),
            Operator::NotLike => write!(f, "NOT LIKE"),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::String(s) => write!(f, "`{}`", s),
            Variable::Int(i) => write!(f, "{}", i),
            Variable::Bool(true) => write!(f, "{}", true),
            Variable::Bool(false) => write!(f, "{}", false),
            Variable::Null => write!(f, "IS NULL"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Asc => write!(f, "ASC"),
            Direction::Desc => write!(f, "DESC"),
        }
    }
}

impl Display for WhereOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhereOperator::And => write!(f, "AND"),
            WhereOperator::Or => write!(f, "OR"),
            WhereOperator::Not => write!(f, "NOT"),
        }
    }
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::Inner => write!(f, "JOIN"),
            JoinType::Left => write!(f, "LEFT JOIN"),
            JoinType::Right => write!(f, "RIGHT JOIN"),
            JoinType::Full => write!(f, "FULL JOIN"),
        }
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionType::Count => write!(f, "COUNT"),
            FunctionType::Max => write!(f, "MAX"),
            FunctionType::Min => write!(f, "MIN"),
            FunctionType::Sum => write!(f, "SUM"),
            FunctionType::Avg => write!(f, "AVG"),
        }
    }
}

impl From<WhereClause> for Clause {
    fn from(where_clause: WhereClause) -> Self {
        Self {
            column: where_clause.column,
            operator: where_clause.operator,
            value: where_clause.value,
        }
    }
}
