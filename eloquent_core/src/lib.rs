//! # Eloquent Core
//!
//! Core query builder expressions and tools.
//!
//! # Quick Start
//!
//! ```rust
//! use eloquent_core::Eloquent;
//!
//! let query = Eloquent::query()
//!     .table("users")
//!     .select("first_name")
//!     .to_sql()
//!     .unwrap();
//!
//! assert_eq!(query, "SELECT `first_name` FROM users;");
//! ```
use std::fmt;

use error::EloquentError;
use expressions::delete_clause::DeleteClause;
use expressions::from_clause::FromClause;
use expressions::group_clause::GroupClauses;
use expressions::insert_clause::InsertClauses;
use expressions::order_clause::OrderClauses;
use expressions::select_clause::SelectClauses;
use expressions::update_clause::UpdateClauses;
use expressions::where_clause::WhereClauses;
use expressions::formattable::Formattable;

mod error;
mod expressions;

/// Eloquent query builder.
///
/// # Example
///
/// ```rust
/// use eloquent_core::Eloquent;
///
/// let query = Eloquent::query();
/// ```
pub struct Eloquent {
    pub from_clause: FromClause,
    pub select_clauses: SelectClauses,
    pub insert_clause: InsertClauses,
    pub update_clause: UpdateClauses,
    pub delete_clause: DeleteClause,
    pub where_clauses: WhereClauses,
    pub group_clauses: GroupClauses,
    pub order_clauses: OrderClauses,
}

/// Used in where/insert/update queries to allow multiple types of variables.
#[derive(Debug)]
pub enum GenericVar
{
    Str(String),
    Int(u32),
    Bool(bool),
    None,
}

/// Used to indicate whether the order by query must be in ascending or descending order.
#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

/// Used to map a value to a certain column.
pub struct Clause {
    pub column: String,
    pub value: GenericVar,
}

impl fmt::Display for GenericVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenericVar::Str(value) => write!(f, "\"{}\"", value),
            GenericVar::Int(value) => write!(f, "{}", value),
            GenericVar::Bool(true) => write!(f, "{}", 1),
            GenericVar::Bool(false) => write!(f, "{}", 0),
            GenericVar::None => write!(f, ""),
        }
    }
}

impl Eloquent {
    pub fn query() -> Eloquent {
        Eloquent {
            from_clause: FromClause {
                table: None,
            },
            select_clauses: SelectClauses {
                clauses: vec![],
            },
            insert_clause: InsertClauses {
                table: None,
                clauses: vec![],
            },
            update_clause: UpdateClauses {
                table: None,
                clauses: vec![],
            },
            delete_clause: DeleteClause {
                table: None,
            },
            where_clauses: WhereClauses {
                clauses: vec![],
            },
            group_clauses: GroupClauses {
                clauses: vec![],
            },
            order_clauses: OrderClauses {
                clauses: vec![],
            },
        }
    }

    pub fn to_sql(&mut self) -> Result<String, error::EloquentError> {
        if self.select_clauses.is_used() && self.insert_clause.is_used() {
            return Err(EloquentError::CombinationSelectAndInsertBindingError);
        }

        if self.from_clause.table.is_some() {
            if self.from_clause.table.is_none() {
                return Err(EloquentError::SelectBindingWithoutTableNameError);
            }

            let select_binding = &self.select_clauses.to_query_format()?;
            let from_binding = &self.from_clause.to_query_format()?;
            let where_binding = &self.where_clauses.to_query_format()?;
            let group_binding = &self.group_clauses.to_query_format()?;
            let order_binding = &self.order_clauses.to_query_format()?;

            return Ok(format!("{} {}{}{}{};",
                select_binding,
                from_binding,
                where_binding,
                group_binding,
                order_binding,
            ));
        } else if self.insert_clause.is_used() {
            let insert_binding = &self.insert_clause.to_query_format()?;

            return Ok(format!("{};",
                insert_binding,
            ));
        } else if self.update_clause.is_used() {
            let update_binding = &self.update_clause.to_query_format()?;
            let where_binding = &self.where_clauses.to_query_format()?;

            return Ok(format!("{}{};",
                update_binding,
                where_binding,
            ));
        } else if self.delete_clause.is_used() {
            let delete_binding = &self.delete_clause.to_query_format()?;
            let where_binding = &self.where_clauses.to_query_format()?;

            return Ok(format!("{}{};",
                delete_binding,
                where_binding,
            ));
        }

        return Err(EloquentError::MissingSelectAndInsertBindingError);
    }
}
