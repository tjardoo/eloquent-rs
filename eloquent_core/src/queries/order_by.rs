use crate::{Order, OrderColumn, QueryBuilder};

impl QueryBuilder {
    /// Add an order by clause to the query in ascending order.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .order_by_asc("origin");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights ORDER BY origin ASC"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .order_by_asc("origin")
    ///     .order_by_desc("destination");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights ORDER BY origin ASC, destination DESC"
    /// );
    /// ```
    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(OrderColumn {
            column: column.to_string(),
            order: Order::Asc,
        });

        self
    }

    /// Add an order by clause to the query in descending order.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .order_by_desc("origin");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights ORDER BY origin DESC"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .order_by_desc("origin")
    ///     .order_by_asc("destination");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights ORDER BY origin DESC, destination ASC"
    /// );
    /// ```
    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(OrderColumn {
            column: column.to_string(),
            order: Order::Desc,
        });

        self
    }
}
