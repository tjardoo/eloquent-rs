use crate::QueryBuilder;

impl QueryBuilder {
    /// Add an offset clause to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .offset(10);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights OFFSET 10"
    /// );
    /// ```
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);

        self
    }
}
