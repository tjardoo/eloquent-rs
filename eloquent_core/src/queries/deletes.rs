use crate::QueryBuilder;

impl QueryBuilder {
    /// Delete rows from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("id", 1)
    ///     .delete();
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "DELETE FROM flights WHERE id = 1"
    /// );
    /// ```
    pub fn delete(mut self) -> Self {
        self.delete = true;

        self
    }
}
