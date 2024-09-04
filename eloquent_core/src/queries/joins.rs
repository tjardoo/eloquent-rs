use crate::{Join, JoinType, QueryBuilder};

impl QueryBuilder {
    fn add_join(
        mut self,
        table: &str,
        left_hand: &str,
        right_hand: &str,
        join_type: JoinType,
    ) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            left_hand: left_hand.to_string(),
            join_type,
            right_hand: right_hand.to_string(),
        });

        self
    }

    /// Inner join two tables together.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .join(
    ///         "airports",
    ///         "flights.origin_airport",
    ///         "airports.code",
    ///     );
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights JOIN airports ON flights.origin_airport = airports.code"
    /// );
    /// ```
    pub fn join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Inner)
    }

    /// Left join two tables together.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .left_join(
    ///         "airports",
    ///         "flights.origin_airport",
    ///         "airports.code",
    ///     );
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights LEFT JOIN airports ON flights.origin_airport = airports.code"
    /// );
    /// ```
    pub fn left_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Left)
    }

    /// Right join two tables together.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .right_join(
    ///         "airports",
    ///         "flights.origin_airport",
    ///         "airports.code",
    ///     );
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights RIGHT JOIN airports ON flights.origin_airport = airports.code"
    /// );
    /// ```
    pub fn right_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Right)
    }

    /// Full join two tables together.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .full_join(
    ///         "airports",
    ///         "flights.origin_airport",
    ///         "airports.code",
    ///     );
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights FULL JOIN airports ON flights.origin_airport = airports.code"
    /// );
    /// ```
    pub fn full_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Full)
    }
}
