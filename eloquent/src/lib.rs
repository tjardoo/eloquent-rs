use builder::QueryBuilder;

pub use eloquent_core::*;

pub struct Eloquent;

impl Eloquent {
    pub fn query() -> QueryBuilder {
        QueryBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_where() {
        let result = Eloquent::query().table("flights").r#where("origin", "AMS");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin = 'AMS'"
        );
    }

    #[test]
    fn test_or_where() {
        let result = Eloquent::query()
            .table("flights")
            .r#where("origin", "AMS")
            .or_where("origin", "FRA");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin = 'AMS' OR origin = 'FRA'"
        );
    }

    #[test]
    fn test_where_not() {
        let result = Eloquent::query()
            .table("flights")
            .where_not("origin", "AMS");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin != 'AMS'"
        );
    }

    #[test]
    fn test_or_where_not() {
        let result = Eloquent::query()
            .table("flights")
            .where_not("origin", "AMS")
            .or_where_not("destination", "AMS");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin != 'AMS' OR destination != 'AMS'"
        );
    }

    #[test]
    fn test_where_gt() {
        let result = Eloquent::query()
            .table("flights")
            .where_gt("flight_duration", 120);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration > 120"
        );
    }

    #[test]
    fn test_or_where_gt() {
        let result = Eloquent::query()
            .table("flights")
            .where_gt("flight_duration", 120)
            .or_where_gt("number_of_passengers ", 200);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration > 120 OR number_of_passengers  > 200"
        );
    }

    #[test]
    fn test_where_gte() {
        let result = Eloquent::query()
            .table("flights")
            .where_gte("flight_duration", 120);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration >= 120"
        );
    }

    #[test]
    fn test_or_where_gte() {
        let result = Eloquent::query()
            .table("flights")
            .where_gte("flight_duration", 120)
            .or_where_gte("number_of_passengers ", 200);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration >= 120 OR number_of_passengers  >= 200"
        );
    }

    #[test]
    fn test_where_lt() {
        let result = Eloquent::query()
            .table("flights")
            .where_lt("flight_duration", 120);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration < 120"
        );
    }

    #[test]
    fn test_or_where_lt() {
        let result = Eloquent::query()
            .table("flights")
            .where_lt("flight_duration", 120)
            .or_where_lt("number_of_passengers ", 200);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration < 120 OR number_of_passengers  < 200"
        );
    }

    #[test]
    fn test_where_lte() {
        let result = Eloquent::query()
            .table("flights")
            .where_lte("flight_duration", 120);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration <= 120"
        );
    }

    #[test]
    fn test_or_where_lte() {
        let result = Eloquent::query()
            .table("flights")
            .where_lte("flight_duration", 120)
            .or_where_lte("number_of_passengers ", 200);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE flight_duration <= 120 OR number_of_passengers  <= 200"
        );
    }

    #[test]
    fn test_where_like() {
        let result = Eloquent::query()
            .table("flights")
            .where_like("airplane_type", "Airbus%");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%'"
        );
    }

    #[test]
    fn test_or_where_like() {
        let result = Eloquent::query()
            .table("flights")
            .where_like("airplane_type", "Airbus%")
            .or_where_like("airplane_type", "Boeing%");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%' OR airplane_type LIKE 'Boeing%'"
        );
    }

    #[test]
    fn test_where_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_in("origin_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_or_where_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_in("origin_airport ", vec!["AMS", "FRA"])
            .or_where_in("destination_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA') OR destination_airport IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_not_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_in("origin_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_or_where_not_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_in("origin_airport ", vec!["AMS", "FRA"])
            .or_where_not_in("destination_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA') OR destination_airport NOT IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_null("departure_time");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NULL"
        );
    }

    #[test]
    fn test_or_where_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_null("departure_time")
            .or_where_null("arrival_time");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NULL OR arrival_time IS NULL"
        );
    }

    #[test]
    fn test_where_not_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL"
        );
    }

    #[test]
    fn test_or_where_not_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time")
            .or_where_not_null("arrival_time");

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL OR arrival_time IS NOT NULL"
        );
    }

    #[test]
    fn test_where_closure() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time")
            .where_closure(|query| query.r#where("origin", "AMS").or_where("origin", "FRA"));

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL AND (origin = 'AMS' OR origin = 'FRA')"
        );
    }

    #[test]
    fn test_or_where_closure() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time")
            .or_where_closure(|query| query.r#where("origin", "AMS").r#where("destination", "FRA"));

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL OR (origin = 'AMS' AND destination = 'FRA')"
        );
    }

    #[test]
    fn test_where_closure_in_closure() {
        let result = Eloquent::query()
            .table("flights")
            .r#where("origin", "AMS")
            .where_closure(|q| {
                q.where_closure(|q| q.where_in("destination", vec!["BKK", "DMK"]))
                    .or_where_closure(|q| q.where_like("aircraft_code", "THA%"))
            });

        assert_eq!(
            result.build_statement(),
            "SELECT * FROM flights WHERE origin = 'AMS' AND (destination IN ('BKK', 'DMK') OR aircraft_code LIKE 'THA%')"
        );
    }
}
