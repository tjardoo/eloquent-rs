pub use eloquent_core::Eloquent;

#[cfg(test)]
mod tests {
    use eloquent_core::Direction;

    use super::*;

    #[test]
    fn it_works() {
        let query = Eloquent::query()
            .table("flights".to_string())
            .select("id".to_string())
            .select("flight_number".to_string())
            .select("destination".to_string())
            .r#where("departure_code".to_string(), "AMS".to_string())
            .r#where("destination".to_string(), "SIN".to_string())
            .where_not("terminal_id".to_string(), "A".to_string())
            .group_by("destination".to_string())
            .order_by("flight_number".to_string(), Direction::Asc)
            .order_by("id".to_string(), Direction::Desc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT id, flight_number, destination FROM flights WHERE departure_code = \"AMS\" AND destination = \"SIN\" AND terminal_id != \"A\" GROUP BY destination ORDER BY flight_number ASC, id DESC;");
    }
}
