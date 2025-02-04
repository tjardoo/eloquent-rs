use crate::{Action, QueryBuilder};

impl QueryBuilder {
    /// Add a condition to the query builder when the given `include` flag is true.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .when(true, |q| {
    ///         q.r#where("destination_airport", "AMS")
    ///     })
    ///     .when(false, |q| {
    ///         q.r#where("destination_airport", "ZRH")
    ///     });
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE destination_airport = 'AMS'"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .when(true, |q| {
    ///         q.insert("destination_airport", "AMS")
    ///     })
    ///     .when(false, |q| {
    ///         q.insert("destination_airport", "ZRH")
    ///     });
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "INSERT INTO flights (destination_airport) VALUES ('AMS')"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("id", "1")
    ///     .when(true, |q| {
    ///         q.update("destination_airport", "BKK")
    ///     })
    ///     .when(false, |q| {
    ///         q.update("destination_airport", "ZRH")
    ///     });
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "UPDATE flights SET destination_airport = 'BKK' WHERE id = '1'"
    /// );
    /// ```
    pub fn when<F>(mut self, include: bool, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        let mut nested_builder = QueryBuilder::new();

        nested_builder = closure(nested_builder);

        if !include {
            return self;
        }

        if nested_builder.get_action() == Action::Insert {
            self.inserts.append(&mut nested_builder.inserts);
        } else if nested_builder.get_action() == Action::Update {
            self.updates.append(&mut nested_builder.updates);
        } else {
            self.conditions.append(&mut nested_builder.conditions);
        }

        self
    }
}
