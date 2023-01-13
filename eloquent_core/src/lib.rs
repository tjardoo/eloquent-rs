use expressions::from_clause::FromClause;
use expressions::select_clause::SelectClauses;
use expressions::where_clause::WhereClauses;
use expressions::formattable::Formattable;

mod error;
mod expressions;

pub struct Eloquent {
    pub from_clause: FromClause,
    pub select_clauses: SelectClauses,
    pub where_clauses: WhereClauses,
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
        }
    }

    pub fn to_sql(&mut self) -> Result<String, error::EloquentError> {
        let select_part = &self.select_clauses.to_query_format()?;
        let from_part = &self.from_clause.to_query_format()?;
        let where_part = &self.where_clauses.to_query_format()?;

        Ok(format!("{} {}{};",
            select_part,
            from_part,
            where_part,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let query = Eloquent::query()
            .table("flights".to_string())
            .select("id".to_string())
            .select("flight_number".to_string())
            .select("destination".to_string())
            .r#where("departure_code".to_string(), "AMS".to_string())
            .r#where("destination_code".to_string(), "SIN".to_string())
            .where_not("terminal_id".to_string(), "A".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT id, flight_number, destination FROM flights WHERE departure_code = \"AMS\" AND destination = \"SIN\" AND terminal_id != \"A\";");
    }
}
