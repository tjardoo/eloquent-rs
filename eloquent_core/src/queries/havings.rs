use crate::{Condition, Having, Logic, Operator, QueryBuilder};

impl QueryBuilder {
    /// Add a having clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration = 300"
    /// );
    /// ```
    pub fn having(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::Equal)
    }

    /// Add a having not clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_not("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration != 300"
    /// );
    /// ```
    pub fn having_not(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::NotEqual)
    }

    /// Add a having greater than clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_gt("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration > 300"
    /// );
    /// ```
    pub fn having_gt(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::GreaterThan)
    }

    /// Add a having greater than or equal to clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_gte("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration >= 300"
    /// );
    /// ```
    pub fn having_gte(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::GreaterThanOrEqual)
    }

    /// Add a having less than clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_lt("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration < 300"
    /// );
    /// ```
    pub fn having_lt(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::LessThan)
    }

    /// Add a having less than or equal to clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_lte("avg_duration", 300);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration <= 300"
    /// );
    /// ```
    pub fn having_lte(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::LessThanOrEqual)
    }

    /// Add a having between clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("flights.origin_airport")
    ///     .select_as("AVG(flights.flight_duration)", "avg_duration")
    ///     .join("airports", "flights.origin_airport", "airports.code")
    ///     .group_by("flights.origin_airport")
    ///     .having_between("avg_duration", 300, 500);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flights.origin_airport, AVG(flights.flight_duration) AS avg_duration FROM flights JOIN airports ON flights.origin_airport = airports.code GROUP BY flights.origin_airport HAVING avg_duration BETWEEN 300 AND 500"
    /// );
    /// ```
    pub fn having_between(mut self, column: &str, value_1: i64, value_2: i64) -> Self {
        self.havings.push(Having {
            conditions: vec![Condition {
                field: column.to_string(),
                operator: Operator::Between,
                logic: Logic::And,
                values: vec![Box::new(value_1), Box::new(value_2)],
            }],
        });

        self
    }

    fn add_having(mut self, column: &str, value: i64, operator: Operator) -> Self {
        self.havings.push(Having {
            conditions: vec![Condition {
                field: column.to_string(),
                operator,
                logic: Logic::And,
                values: vec![Box::new(value)],
            }],
        });

        self
    }
}
