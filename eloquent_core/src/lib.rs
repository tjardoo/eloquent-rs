//! # Eloquent Core
//!
//! `eloquent_core` is a library for building SQL queries in Rust.
//!

use std::fmt::Display;

use builder::Bindings;

mod builder;
mod compiler;

pub struct Eloquent {
    pub bindings: Bindings,
}

impl Eloquent {
    pub fn new() -> Self {
        Self {
            bindings: Bindings {
                select: vec![],
                insert: vec![],
                update: vec![],
                from: None,
                // join: vec![],
                r#where: vec![],
                // group_by: vec![],
                // having: vec![],
                // order_by: vec![],
                is_delete: false,
                limit: None,
                offset: None,
            },
        }
    }
}

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

#[derive(Debug, Clone)]
pub enum Variable {
    String(String),
    Int(u32),
    Bool(bool),
    Null,
}

pub struct Clause {
    pub column: String,
    pub operator: Operator,
    pub value: Variable,
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
