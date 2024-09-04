use crate::{Columnable, Function, QueryBuilder, Select, Selectable, ToSql};

impl QueryBuilder {
    /// Select single or multiple columns from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select("origin");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT origin FROM flights"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select(vec!["origin", "destination"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT origin, destination FROM flights"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, SubqueryBuilder};
    ///
    /// let subquery = SubqueryBuilder::new()
    ///     .table("flights")
    ///     .select_avg("duration_in_min", "avg_duration_in_min");
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_as(subquery, "avg_duration");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT (SELECT AVG(duration_in_min) AS avg_duration_in_min FROM flights) AS avg_duration FROM flights"
    /// );
    /// ```
    pub fn select<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.selects.push(Select {
                function: None,
                column: column.to_string(),
                alias: None,
            });
        }

        self
    }

    /// Select a single column from the table with an alias.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_as("origin", "from")
    ///     .select_as("destination", "to");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT origin AS from, destination AS to FROM flights"
    /// );
    /// ```
    pub fn select_as<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: None,
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select a raw SQL expression from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_raw("flight_duration * ? as delay_in_min", vec![5]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flight_duration * 5 as delay_in_min FROM flights"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_raw(
    ///         "flight_duration * ? as delay_in_min, delay_in_min * ? as delay_in_hr",
    ///         vec![5, 60],
    ///     );
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT flight_duration * 5 as delay_in_min, delay_in_min * 60 as delay_in_hr FROM flights"
    /// );
    /// ```
    pub fn select_raw(mut self, raw: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let mut formatted_raw = raw.to_string();
        for value in values {
            formatted_raw = formatted_raw.replacen('?', &value.to_sql().unwrap(), 1);
        }

        self.selects.push(Select {
            function: None,
            column: formatted_raw.to_string(),
            alias: None,
        });

        self
    }

    /// Select the count of all rows from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_count("id", "id_count");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT COUNT(id) AS id_count FROM flights"
    /// );
    /// ```
    pub fn select_count<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Count),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select the minimum value of a column from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_min("flight_duration", "flight_duration_min");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT MIN(flight_duration) AS flight_duration_min FROM flights"
    /// );
    /// ```
    pub fn select_min<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Min),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select the maximum value of a column from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_max("flight_duration", "flight_duration_max");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT MAX(flight_duration) AS flight_duration_max FROM flights"
    /// );
    /// ```
    pub fn select_max<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Max),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select the average value of a column from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_avg("flight_duration", "flight_duration_avg");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT AVG(flight_duration) AS flight_duration_avg FROM flights"
    /// );
    /// ```
    pub fn select_avg<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Avg),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select the sum of all values in a column from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_sum("flight_duration", "flight_duration_sum");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT SUM(flight_duration) AS flight_duration_sum FROM flights"
    /// );
    /// ```
    pub fn select_sum<T>(mut self, column: T, alias: &str) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Sum),
            column: column.to_select_column(),
            alias: Some(alias.to_string()),
        });

        self
    }

    /// Select the distinct values of a column from the table.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .select_distinct("origin");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT DISTINCT origin FROM flights"
    /// );
    /// ```
    pub fn select_distinct<T>(mut self, column: T) -> Self
    where
        T: Selectable,
    {
        self.selects.push(Select {
            function: Some(Function::Distinct),
            column: column.to_select_column(),
            alias: None,
        });

        self
    }
}
