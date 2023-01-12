pub use eloquent_core::Eloquent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let query = Eloquent::query()
            .table("flights".to_string())
            .r#where("flight_number".to_string(), "KL0835".to_string())
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT * FROM flights WHERE flight_number = \"KL0835\";");
    }
}
