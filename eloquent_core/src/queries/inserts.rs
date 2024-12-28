use crate::{QueryBuilder, ToSql};

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
        self.inserts
            .entry(column.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(value));

        self
    }
    
    /// Insert single or multiple columns into the table.
    /// 
    /// ```
    /// let query = Eloquent::query()
    ///     .table("users")
    ///     .insert_many(rows);
    /// 
    /// assert_eq!(
    ///         query.sql().unwrap(),
    ///         "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com'), ('Bob', 'bob@example.com')"
    ///     );
    /// ```
    pub fn insert_many(mut self, rows: Vec<Vec<(&str, impl ToSql + 'static)>>) -> Self {
        rows.into_iter().for_each(|row| {
            row.into_iter().for_each(|(column, value)| {
                self.inserts
                    .entry(column.to_string())
                    .or_insert_with(Vec::new)
                    .push(Box::new(value));
            });
        });

        self
    }
}
