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

    use error::EloquentError;

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
    fn test_select_raw() {
        let result = Eloquent::query()
            .table("flights")
            .select_raw("flight_duration * ? as delay_in_min", vec![5]);

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flight_duration * 5 as delay_in_min FROM flights"
        );
    }

    #[test]
    fn test_select_raw_multiple() {
        let result = Eloquent::query().table("flights").select_raw(
            "flight_duration * ? as delay_in_min, delay_in_min * ? as delay_in_hr",
            vec![5, 60],
        );

        assert_eq!(
            result.sql().unwrap(),
            "SELECT flight_duration * 5 as delay_in_min, delay_in_min * 60 as delay_in_hr FROM flights"
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
    fn test_insert() {
        let result = Eloquent::query()
            .table("flights")
            .insert("origin_airport", "AMS")
            .insert("destination_airport", "FRA");

        assert_eq!(
            result.sql().unwrap(),
            "INSERT INTO flights (origin_airport, destination_airport) VALUES ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_insert_with_condition() {
        let result = Eloquent::query()
            .table("flights")
            .insert("origin_airport", "AMS")
            .insert("destination_airport", "FRA");

        assert_eq!(
            result.sql().unwrap(),
            "INSERT INTO flights (origin_airport, destination_airport) VALUES ('AMS', 'FRA')"
        );
    }

    #[test]
    fn test_update() {
        let result = Eloquent::query()
            .table("flights")
            .update("origin_airport", "AMS")
            .update("destination_airport", "FRA");

        assert_eq!(
            result.sql().unwrap(),
            "UPDATE flights SET origin_airport = 'AMS', destination_airport = 'FRA'"
        );
    }

    #[test]
    fn test_update_with_condition() {
        let result = Eloquent::query()
            .table("flights")
            .update("origin_airport", "AMS")
            .update("destination_airport", "FRA")
            .r#where("id", 1);

        assert_eq!(
            result.sql().unwrap(),
            "UPDATE flights SET origin_airport = 'AMS', destination_airport = 'FRA' WHERE id = 1"
        );
    }

    #[test]
    fn test_delete() {
        let result = Eloquent::query().table("flights").delete();

        assert_eq!(result.sql().unwrap(), "DELETE FROM flights");
    }

    #[test]
    fn test_delete_with_condition() {
        let result = Eloquent::query()
            .table("flights")
            .r#where("origin", "AMS")
            .delete();

        assert_eq!(
            result.sql().unwrap(),
            "DELETE FROM flights WHERE origin = 'AMS'"
        );
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
    fn test_missing_table() {
        let result = Eloquent::query().sql();

        match result {
            Err(EloquentError::MissingTable) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_duplicated_column_names() {
        let result = Eloquent::query()
            .table("flights")
            .select("origin")
            .select("origin")
            .sql();

        match result {
            Err(EloquentError::DuplicatedColumnNames(column)) => assert_eq!(column, "origin"),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_having_clause_without_aggregate_function() {
        let result = Eloquent::query()
            .table("flights")
            .having("origin", 300)
            .sql();

        match result {
            Err(EloquentError::HavingClauseWithoutAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_group_by_without_selected_or_aggregate_function() {
        let result = Eloquent::query().table("flights").group_by("origin").sql();

        match result {
            Err(EloquentError::GroupByWithNonSelectedOrAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_order_by_without_selected_or_aggregate_function() {
        let result = Eloquent::query()
            .table("flights")
            .select("destination")
            .order_by_asc("origin")
            .sql();

        match result {
            Err(EloquentError::OrderByWithNonSelectedOrAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_missing_placeholder() {
        let result = Eloquent::query()
            .select_raw(
                "flight_duration * ? as delay_in_min, delay_in_min * ? as delay_in_hr",
                vec![5],
            )
            .table("flights")
            .sql();

        match result {
            Err(EloquentError::MissingPlaceholders) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_disabled_checks() {
        let result = Eloquent::query().table("flights").disable_checks().sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights");
    }

    #[test]
    fn test_cannot_apply_clause_on_insert() {
        let result = Eloquent::query()
            .table("flights")
            .insert("origin_airport", "AMS")
            .r#where("origin_airport", "FRA")
            .sql();

        match result {
            Err(EloquentError::CannotApplyClauseOnInsert(clause)) => {
                assert_eq!(clause, "WHERE")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_cannot_apply_clause_on_update() {
        let result = Eloquent::query()
            .table("flights")
            .join("airports", "flights.origin_airport", "airports.code")
            .update("origin_airport", "AMS")
            .sql();

        match result {
            Err(EloquentError::CannotApplyClauseOnUpdate(clause)) => {
                assert_eq!(clause, "JOIN")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_cannot_apply_clause_on_delete() {
        let result = Eloquent::query()
            .table("flights")
            .join("airports", "flights.origin_airport", "airports.code")
            .delete()
            .sql();

        match result {
            Err(EloquentError::CannotApplyClauseOnDelete(clause)) => {
                assert_eq!(clause, "JOIN")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
