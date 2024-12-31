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
        self.add_insert(column, Box::new(value));

        self
    }

    /// Insert single or multiple rows into the table.
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, ToSql, eloquent_sql_row};
    ///
    /// let rows = vec![
    ///     eloquent_sql_row! {
    ///         "name" => "Alice",
    ///         "email" => "alice@example.com",
    ///         "age" => 21,
    ///         "is_active" => true,
    ///     },
    ///     eloquent_sql_row! {
    ///         "name" => "Bob",
    ///         "email" => "bob@example.com",
    ///         "age" => 22,
    ///         "is_active" => false,
    ///     },
    /// ];
    /// let query = QueryBuilder::new()
    ///     .table("users")
    ///     .insert_many(rows);
    ///
    /// assert_eq!(
    ///     query.sql().unwrap(),
    ///     "INSERT INTO users (name, email, age, is_active) VALUES ('Alice', 'alice@example.com', 21, true), ('Bob', 'bob@example.com', 22, false)"
    /// );
    /// ```
    pub fn insert_many(mut self, rows: Vec<Vec<(&str, Box<dyn ToSql>)>>) -> Self {
        rows.into_iter().for_each(|row| self.add_row(row));

        self
    }

    fn add_insert(&mut self, column: &str, value: Box<dyn ToSql>) {
        if let Some(insert) = self.inserts.iter_mut().find(|i| i.column == column) {
            insert.values.push(value);
        } else {
            self.inserts.push(Insert {
                column: column.to_string(),
                values: vec![value],
            });
        }
    }

    fn add_row(&mut self, row: Vec<(&str, Box<dyn ToSql>)>) {
        row.into_iter()
            .for_each(|(column, value)| self.add_insert(column, value));
    }
}
