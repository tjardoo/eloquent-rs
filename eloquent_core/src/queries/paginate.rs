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
        per_page: u64,
    ) -> Self {
        self.paginate = Some(Paginate {
            column: column.to_string(),
            last_id: last_id.map(|id| Box::new(id) as Box<dyn ToSql>),
            per_page,
        });

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::QueryBuilder;

    #[test]
    fn test_paginate_with_where() {
        let result = QueryBuilder::new()
            .table("departures")
            .r#where("origin", "AMS")
            .paginate("id", Some(1000), 25)
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM departures WHERE id > 1000 AND origin = 'AMS' ORDER BY id ASC LIMIT 25"
        );
    }

    #[test]
    fn test_paginate_first_page_with_where() {
        let result = QueryBuilder::new()
            .table("departures")
            .r#where("origin", "AMS")
            .paginate::<u64>("id", None, 25)
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM departures WHERE origin = 'AMS' ORDER BY id ASC LIMIT 25"
        );
    }

    #[test]
    fn test_paginate_with_or_conditions_keeps_cursor() {
        let result = QueryBuilder::new()
            .table("departures")
            .r#where("origin", "AMS")
            .or_where("origin", "FRA")
            .paginate("id", Some(1000), 25)
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM departures WHERE id > 1000 AND (origin = 'AMS' OR origin = 'FRA') ORDER BY id ASC LIMIT 25"
        );
    }
}
