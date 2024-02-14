//! # Eloquent Core
//!
//! Eloquent Core is the core library for the Eloquent libary. It provides the core functionality for building SQL queries.
//!

use builder::Bindings;

mod builder;
mod closures;
mod compiler;
mod shared;
mod traits;

/// The main struct for Eloquent to build SQL queries.
pub struct Eloquent {
    bindings: Bindings,
}

impl Eloquent {
    /// Create a new instance of Eloquent with the given table name.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select("id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT id FROM users");
    /// ```
    pub fn table(name: &str) -> Self {
        Self {
            bindings: Bindings {
                select: vec![],
                insert: vec![],
                update: vec![],
                table: name.to_string(),
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

/// The operator to use in a "where" clause.
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
    In,
    NotIn,
}

/// The value to use in a "where" clause.
#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    String(String),
    Int(u32),
    Bool(bool),
    Null,
    Array(Vec<ArrayVariable>),
}

/// The array value to use in a "where" clause.
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayVariable {
    String(String),
    Int(u32),
}

/// The direction to use in an "order by" clause.
pub enum Direction {
    Asc,
    Desc,
}
