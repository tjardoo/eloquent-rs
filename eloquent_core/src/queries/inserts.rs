use crate::{Insert, QueryBuilder, ToSql};

impl QueryBuilder {
    /// Insert single or multiple columns into the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .insert("origin_airport", "AMS")
    ///     .insert("destination_airport", "FRA");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "INSERT INTO flights (origin_airport, destination_airport) VALUES ('AMS', 'FRA')"
    /// );
    /// ```
    pub fn insert(mut self, column: &str, value: impl ToSql + 'static) -> Self {
        self.inserts.push(Insert {
            column: column.to_string(),
            value: Box::new(value),
        });

        self
    }
}
