use expressions::from_clause::FromClause;
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
            order_clauses: OrderClauses {
                clauses: vec![],
            },
        }
    }

    pub fn to_sql(&mut self) -> Result<String, error::EloquentError> {
        let select_part = &self.select_clauses.to_query_format()?;
        let from_part = &self.from_clause.to_query_format()?;
        let where_part = &self.where_clauses.to_query_format()?;
        let order_part = &self.order_clauses.to_query_format()?;

        Ok(format!("{} {}{}{};",
            select_part,
            from_part,
            where_part,
            order_part,
        ))
    }
}
