use crate::{Paginate, QueryBuilder, ToSql};

impl QueryBuilder {
    /// Paginate the query results.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("departures")
    ///     .paginate::<u64>("id", None, 25);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM departures ORDER BY id ASC LIMIT 25"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("departures")
    ///     .paginate("id", Some(1000), 25);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM departures WHERE id > 1000 ORDER BY id ASC LIMIT 25"
    /// );
    /// ```
    pub fn paginate<T: ToSql + 'static>(
        mut self,
        column: &str,
        last_id: Option<T>,
        per_page: i64,
    ) -> Self {
        self.paginate = Some(Paginate {
            column: column.to_string(),
            last_id: last_id.map(|id| Box::new(id) as Box<dyn ToSql>),
            per_page: per_page as u64,
        });

        self
    }
}
