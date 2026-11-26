use crate::{InsertRow, QueryBuilder, ToSql};

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
        let value: Box<dyn ToSql> = Box::new(value);

        match self.inserts.last_mut() {
            Some(row) if !row.has_column(column) => row.values.push((column.to_string(), value)),
            _ => self.inserts.push(InsertRow {
                values: vec![(column.to_string(), value)],
            }),
        }

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
        self.inserts.extend(rows.into_iter().map(|row| {
            InsertRow {
                values: row
                    .into_iter()
                    .map(|(column, value)| (column.to_string(), value))
                    .collect(),
            }
        }));

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::QueryBuilder;

    #[test]
    fn test_insert_repeated_column_starts_new_row() {
        let result = QueryBuilder::new()
            .table("flights")
            .insert("origin_airport", "AMS")
            .insert("destination_airport", "FRA")
            .insert("origin_airport", "BKK")
            .insert("destination_airport", "SIN")
            .sql();

        assert_eq!(
            result.unwrap(),
            "INSERT INTO flights (origin_airport, destination_airport) VALUES ('AMS', 'FRA'), ('BKK', 'SIN')"
        );
    }
}
