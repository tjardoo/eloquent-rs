use crate::{Columnable, QueryBuilder};

impl QueryBuilder {
    /// Add a group by clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("origin")
    ///     .select_avg("flight_duration", "flight_duration_avg")
    ///     .group_by("origin");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT origin, AVG(flight_duration) AS flight_duration_avg FROM flights GROUP BY origin"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select(vec!["origin", "destination"])
    ///     .select_avg("flight_duration", "flight_duration_avg")
    ///     .group_by(vec!["origin", "destination"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT origin, destination, AVG(flight_duration) AS flight_duration_avg FROM flights GROUP BY origin, destination"
    /// );
    /// ```
    pub fn group_by<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.group_by.push(column.to_string());
        }

        self
    }
}
