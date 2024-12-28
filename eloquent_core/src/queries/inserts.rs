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
        self.add_insert(column, value);

        self
    }

    /// Insert single or multiple rows into the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let rows = vec![
    ///     vec![("name", "Alice"), ("email", "alice@example.com")],
    ///     vec![("name", "Bob"), ("email", "bob@example.com")],
    /// ];
    ///
    /// let query = QueryBuilder::new()
    ///     .table("users")
    ///     .insert_many(rows);
    ///
    /// assert_eq!(
    ///     query.sql().unwrap(),
    ///     "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com'), ('Bob', 'bob@example.com')"
    /// );
    /// ```
    pub fn insert_many(mut self, rows: Vec<Vec<(&str, impl ToSql + 'static)>>) -> Self {
        rows.into_iter().for_each(|row| self.add_row(row));

        self
    }

    fn add_insert(&mut self, column: &str, value: impl ToSql + 'static) {
        if let Some(insert) = self.inserts.iter_mut().find(|i| i.column == column) {
            insert.values.push(Box::new(value));
        } else {
            self.inserts.push(Insert {
                column: column.to_string(),
                values: vec![Box::new(value)],
            });
        }
    }

    fn add_row(&mut self, row: Vec<(&str, impl ToSql + 'static)>) {
        row.into_iter()
            .for_each(|(column, value)| self.add_insert(column, value));
    }
}
