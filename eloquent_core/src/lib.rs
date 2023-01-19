use std::fmt;

use error::EloquentError;
use expressions::from_clause::FromClause;
use expressions::group_clause::GroupClauses;
use expressions::insert_clause::InsertClauses;
use expressions::order_clause::OrderClauses;
use expressions::select_clause::SelectClauses;
use expressions::where_clause::WhereClauses;
use expressions::formattable::Formattable;

mod error;
mod expressions;

pub struct Eloquent {
    pub from_clause: FromClause,
    pub select_clauses: SelectClauses,
    pub insert_clause: InsertClauses,
    pub where_clauses: WhereClauses,
    pub group_clauses: GroupClauses,
    pub order_clauses: OrderClauses,
}

#[derive(Debug)]
pub enum GenericVar
{
    Str(String),
    Int(u32),
    Bool(bool)
}

#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}

pub struct InsertClause {
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
        }

        return Err(EloquentError::MissingSelectAndInsertBindingError);
    }
}
