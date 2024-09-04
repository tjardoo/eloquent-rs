use crate::QueryBuilder;

impl QueryBuilder {
    /// Add a limit clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .limit(10);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights LIMIT 10"
    /// );
    /// ```
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }
}
