use crate::{QueryBuilder, ToSql, Update};

impl QueryBuilder {
    /// Update single or multiple columns in the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .update("origin_airport", "AMS")
    ///     .update("destination_airport", "FRA")
    ///     .r#where("id", 1);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "UPDATE flights SET origin_airport = 'AMS', destination_airport = 'FRA' WHERE id = 1"
    /// );
    /// ```
    pub fn update(mut self, column: &str, value: impl ToSql + 'static) -> Self {
        self.updates.push(Update {
            column: column.to_string(),
            value: Box::new(value),
        });

        self
    }
}
