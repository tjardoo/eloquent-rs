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
    fn test_select_single_column() {
        let result = Eloquent::query().table("flights").select("origin");

        assert_eq!(result.build(), "SELECT origin FROM flights");
    }

    #[test]
    fn test_select_multiple_columns() {
        let result = Eloquent::query()
            .table("flights")
            .select(vec!["origin", "destination"]);

        assert_eq!(result.build(), "SELECT origin, destination FROM flights");
    }

    #[test]
    fn test_select_as() {
        let result = Eloquent::query()
            .table("flights")
            .select_as("origin", "from")
            .select_as("destination", "to");

        assert_eq!(
            result.build(),
            "SELECT origin AS from, destination AS to FROM flights"
        );
    }

    #[test]
    fn test_select_count() {
        let result = Eloquent::query().table("flights").select_count("id");

        assert_eq!(result.build(), "SELECT COUNT(id) FROM flights");
    }

    #[test]
    fn test_select_min() {
        let result = Eloquent::query()
            .table("flights")
            .select_min("flight_duration");

        assert_eq!(result.build(), "SELECT MIN(flight_duration) FROM flights");
    }

    #[test]
    fn test_select_max() {
        let result = Eloquent::query()
            .table("flights")
            .select_max("flight_duration");

        assert_eq!(result.build(), "SELECT MAX(flight_duration) FROM flights");
    }

    #[test]
    fn test_select_avg() {
        let result = Eloquent::query()
            .table("flights")
            .select_avg("flight_duration");

        assert_eq!(result.build(), "SELECT AVG(flight_duration) FROM flights");
    }

    #[test]
    fn test_select_sum() {
        let result = Eloquent::query()
            .table("flights")
            .select_sum("flight_duration");

        assert_eq!(result.build(), "SELECT SUM(flight_duration) FROM flights");
    }

    #[test]
    fn test_where() {
        let result = Eloquent::query().table("flights").r#where("origin", "AMS");

        assert_eq!(result.build(), "SELECT * FROM flights WHERE origin = 'AMS'");
    }

    #[test]
    fn test_or_where() {
        let result = Eloquent::query()
            .table("flights")
            .r#where("origin", "AMS")
            .or_where("origin", "FRA");

        assert_eq!(
            result.build(),
            "SELECT * FROM flights WHERE origin = 'AMS' OR origin = 'FRA'"
        );
    }

    #[test]
    fn test_where_not() {
        let result = Eloquent::query()
            .table("flights")
            .where_not("origin", "AMS");

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE origin != 'AMS' OR destination != 'AMS'"
        );
    }

    #[test]
    fn test_where_gt() {
        let result = Eloquent::query()
            .table("flights")
            .where_gt("flight_duration", 120);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE flight_duration > 120 OR number_of_passengers  > 200"
        );
    }

    #[test]
    fn test_where_gte() {
        let result = Eloquent::query()
            .table("flights")
            .where_gte("flight_duration", 120);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE flight_duration >= 120 OR number_of_passengers  >= 200"
        );
    }

    #[test]
    fn test_where_lt() {
        let result = Eloquent::query()
            .table("flights")
            .where_lt("flight_duration", 120);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE flight_duration < 120 OR number_of_passengers  < 200"
        );
    }

    #[test]
    fn test_where_lte() {
        let result = Eloquent::query()
            .table("flights")
            .where_lte("flight_duration", 120);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE flight_duration <= 120 OR number_of_passengers  <= 200"
        );
    }

    #[test]
    fn test_where_like() {
        let result = Eloquent::query()
            .table("flights")
            .where_like("airplane_type", "Airbus%");

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%' OR airplane_type LIKE 'Boeing%'"
        );
    }

    #[test]
    fn test_where_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_in("origin_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA') OR destination_airport IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_not_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_in("origin_airport ", vec!["AMS", "FRA"]);

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA') OR destination_airport NOT IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_null("departure_time");

        assert_eq!(
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE departure_time IS NULL OR arrival_time IS NULL"
        );
    }

    #[test]
    fn test_where_not_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time");

        assert_eq!(
            result.build(),
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
            result.build(),
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
            result.build(),
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
            result.build(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL OR (origin = 'AMS' AND destination = 'FRA')"
        );
    }
}
