pub use eloquent_core::*;

pub struct Eloquent;

impl Eloquent {
    pub fn query() -> QueryBuilder {
        QueryBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_select_single_column() {
        let result = Eloquent::query().table("flights").select("origin");

        assert_eq!(result.sql().unwrap(), "SELECT origin FROM flights");
    }

    #[test]
    fn test_select_multiple_columns() {
        let result = Eloquent::query()
            .table("flights")
            .select(vec!["origin", "destination"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT origin, destination FROM flights"
        );
    }

    #[test]
    fn test_select_as() {
        let result = Eloquent::query()
            .table("flights")
            .select_as("origin", "from")
            .select_as("destination", "to");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT origin AS from, destination AS to FROM flights"
        );
    }

    #[test]
    fn test_select_count() {
        let result = Eloquent::query().table("flights").select_count("id");

        assert_eq!(result.sql().unwrap(), "SELECT COUNT(id) FROM flights");
    }

    #[test]
    fn test_select_min() {
        let result = Eloquent::query()
            .table("flights")
            .select_min("flight_duration");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT MIN(flight_duration) FROM flights"
        );
    }

    #[test]
    fn test_select_max() {
        let result = Eloquent::query()
            .table("flights")
            .select_max("flight_duration");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT MAX(flight_duration) FROM flights"
        );
    }

    #[test]
    fn test_select_avg() {
        let result = Eloquent::query()
            .table("flights")
            .select_avg("flight_duration");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT AVG(flight_duration) FROM flights"
        );
    }

    #[test]
    fn test_select_sum() {
        let result = Eloquent::query()
            .table("flights")
            .select_sum("flight_duration");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT SUM(flight_duration) FROM flights"
        );
    }

    #[test]
    fn test_select_distinct() {
        let result = Eloquent::query().table("flights").select_distinct("origin");

        assert_eq!(result.sql().unwrap(), "SELECT DISTINCT origin FROM flights");
    }

    #[test]
    fn test_where() {
        let result = Eloquent::query().table("flights").r#where("origin", "AMS");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin = 'AMS'"
        );
    }

    #[test]
    fn test_or_where() {
        let result = Eloquent::query()
            .table("flights")
            .r#where("origin", "AMS")
            .or_where("destination", "FRA");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin = 'AMS' OR destination = 'FRA'"
        );
    }

    #[test]
    fn test_where_not() {
        let result = Eloquent::query()
            .table("flights")
            .where_not("origin", "AMS");

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin != 'AMS' OR destination != 'AMS'"
        );
    }

    #[test]
    fn test_where_gt() {
        let result = Eloquent::query()
            .table("flights")
            .where_gt("flight_duration", 120);

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE flight_duration > 120 OR number_of_passengers  > 200"
        );
    }

    #[test]
    fn test_where_gte() {
        let result = Eloquent::query()
            .table("flights")
            .where_gte("flight_duration", 120);

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE flight_duration >= 120 OR number_of_passengers  >= 200"
        );
    }

    #[test]
    fn test_where_lt() {
        let result = Eloquent::query()
            .table("flights")
            .where_lt("flight_duration", 120);

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE flight_duration < 120 OR number_of_passengers  < 200"
        );
    }

    #[test]
    fn test_where_lte() {
        let result = Eloquent::query()
            .table("flights")
            .where_lte("flight_duration", 120);

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE flight_duration <= 120 OR number_of_passengers  <= 200"
        );
    }

    #[test]
    fn test_where_like() {
        let result = Eloquent::query()
            .table("flights")
            .where_like("airplane_type", "Airbus%");

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%' OR airplane_type LIKE 'Boeing%'"
        );
    }

    #[test]
    fn test_where_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_in("origin_airport", vec!["AMS", "FRA"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_or_where_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_in("origin_airport", vec!["AMS", "FRA"])
            .or_where_in("destination_airport", vec!["AMS", "FRA"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA') OR destination_airport IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_not_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_in("origin_airport", vec!["AMS", "FRA"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_or_where_not_in() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_in("origin_airport", vec!["AMS", "FRA"])
            .or_where_not_in("destination_airport", vec!["AMS", "FRA"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA') OR destination_airport NOT IN ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_where_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_null("departure_time")
            .where_null(vec!["arrival_time", "gate_number"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE departure_time IS NULL AND arrival_time IS NULL AND gate_number IS NULL"
        );
    }

    #[test]
    fn test_or_where_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_null("departure_time")
            .or_where_null("arrival_time");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE departure_time IS NULL OR arrival_time IS NULL"
        );
    }

    #[test]
    fn test_where_not_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time")
            .where_not_null(vec!["arrival_time", "gate_number"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL AND arrival_time IS NOT NULL AND gate_number IS NOT NULL"
        );
    }

    #[test]
    fn test_or_where_not_null() {
        let result = Eloquent::query()
            .table("flights")
            .where_not_null("departure_time")
            .or_where_not_null("arrival_time");

        assert_eq!(
            result.sql().unwrap(),
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
            result.sql().unwrap(),
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
            result.sql().unwrap(),
            "SELECT * FROM flights WHERE departure_time IS NOT NULL OR (origin = 'AMS' AND destination = 'FRA')"
        );
    }

    #[test]
    fn test_join() {
        let result = Eloquent::query().table("flights").join(
            "airports",
            "flights.origin_airport",
            "airports.code",
        );

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights JOIN airports ON flights.origin_airport = airports.code"
        );
    }

    #[test]
    fn test_left_join() {
        let result = Eloquent::query().table("flights").left_join(
            "airports",
            "flights.origin_airport",
            "airports.code",
        );

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights LEFT JOIN airports ON flights.origin_airport = airports.code"
        );
    }

    #[test]
    fn test_right_join() {
        let result = Eloquent::query().table("flights").right_join(
            "airports",
            "flights.origin_airport",
            "airports.code",
        );

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights RIGHT JOIN airports ON flights.origin_airport = airports.code"
        );
    }

    #[test]
    fn test_full_join() {
        let result = Eloquent::query().table("flights").full_join(
            "airports",
            "flights.origin_airport",
            "airports.code",
        );

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights FULL JOIN airports ON flights.origin_airport = airports.code"
        );
    }

    #[test]
    fn test_group_by() {
        let result = Eloquent::query()
            .table("flights")
            .select("origin")
            .select_avg("flight_duration")
            .group_by("origin");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT origin, AVG(flight_duration) FROM flights GROUP BY origin"
        );
    }

    #[test]
    fn test_group_by_multiple() {
        let result = Eloquent::query()
            .table("flights")
            .select(vec!["origin", "destination"])
            .select_avg("flight_duration")
            .group_by(vec!["origin", "destination"]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT origin, destination, AVG(flight_duration) FROM flights GROUP BY origin, destination"
        );
    }

    #[test]
    fn test_having() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration = 300"
        );
    }

    #[test]
    fn test_having_not() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having_not("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration != 300"
        );
    }

    #[test]
    fn test_having_gt() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having_gt("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration > 300"
        );
    }

    #[test]
    fn test_having_gte() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having_gte("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration >= 300"
        );
    }

    #[test]
    fn test_having_lt() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having_lt("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration < 300"
        );
    }

    #[test]
    fn test_having_lte() {
        let result = Eloquent::query()
            .table("flights")
            .select("flights.origin_airport")
            .select_as("AVG(flights.flight_duration)", "avg_duration")
            .join("airports", "flights.origin_airport", "airports.code")
            .group_by("flights.origin_airport")
            .having_lte("avg_duration", 300);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration <= 300"
        );
    }

    #[test]
    fn test_order_by_asc() {
        let result = Eloquent::query().table("flights").order_by_asc("origin");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights ORDER BY origin ASC"
        );
    }

    #[test]
    fn test_order_by_desc() {
        let result = Eloquent::query().table("flights").order_by_desc("origin");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights ORDER BY origin DESC"
        );
    }

    #[test]
    fn test_order_by_multiple() {
        let result = Eloquent::query()
            .table("flights")
            .order_by_asc("origin")
            .order_by_desc("destination");

        assert_eq!(
            result.sql().unwrap(),
            "SELECT * FROM flights ORDER BY origin ASC, destination DESC"
        );
    }

    #[test]
    fn test_limit() {
        let result = Eloquent::query().table("flights").limit(10);

        assert_eq!(result.sql().unwrap(), "SELECT * FROM flights LIMIT 10");
    }

    #[test]
    fn test_offset() {
        let result = Eloquent::query().table("flights").offset(10);

        assert_eq!(result.sql().unwrap(), "SELECT * FROM flights OFFSET 10");
    }

    #[test]
    #[should_panic]
    fn test_duplicated_column_names() {
        let _result = Eloquent::query()
            .table("flights")
            .select("origin")
            .select("origin")
            .sql()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_having_clause_without_aggregate_function() {
        let _result = Eloquent::query()
            .table("flights")
            .having("origin", 300)
            .sql()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_group_by_without_selected_or_aggregate_function() {
        let _result = Eloquent::query()
            .table("flights")
            .group_by("origin")
            .sql()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_order_by_without_selected_or_aggregate_function() {
        let _result = Eloquent::query()
            .table("flights")
            .select("destination")
            .order_by_asc("origin")
            .sql()
            .unwrap();
    }
}
