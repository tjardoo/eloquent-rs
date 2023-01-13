pub use eloquent_core::Eloquent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let query = Eloquent::query()
            .table("flights".to_string())
            .r#where("flight_number".to_string(), "KL0835".to_string())
            .r#where("destination".to_string(), "Singapore".to_string())
            .where_not("terminal_id".to_string(), "A".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM flights WHERE flight_number = \"KL0835\" AND destination = \"Singapore\" AND terminal_id != \"A\";");
    }
}
