use expressions::from_clause::FromClause;
use expressions::group_clause::GroupClauses;
use expressions::order_clause::OrderClauses;
use expressions::select_clause::SelectClauses;
use expressions::where_clause::WhereClauses;
use expressions::formattable::Formattable;

mod error;
mod expressions;

pub struct Eloquent {
    pub from_clause: FromClause,
    pub select_clauses: SelectClauses,
    pub where_clauses: WhereClauses,
    pub group_clauses: GroupClauses,
    pub order_clauses: OrderClauses,
}

#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
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
        let select_binding = &self.select_clauses.to_query_format()?;
        let from_binding = &self.from_clause.to_query_format()?;
        let where_binding = &self.where_clauses.to_query_format()?;
        let group_binding = &self.group_clauses.to_query_format()?;
        let order_binding = &self.order_clauses.to_query_format()?;

        Ok(format!("{} {}{}{}{};",
            select_binding,
            from_binding,
            where_binding,
            group_binding,
            order_binding,
        ))
    }
}
