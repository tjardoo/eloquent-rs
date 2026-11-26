use crate::QueryBuilder;

impl QueryBuilder {
    /// Apply the closure to the query builder when the given `include` flag is true.
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
    pub fn when<F>(self, include: bool, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if include {
            closure(self)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    fn flights_by_airline(airline: Option<&'static str>) -> Result<String, EloquentError> {
        QueryBuilder::new()
            .table("flights")
            .when(airline.is_some(), |q| {
                q.r#where("airline", airline.unwrap())
            })
            .sql()
    }

    #[test]
    fn test_when_false_does_not_call_closure() {
        assert_eq!(flights_by_airline(None).unwrap(), "SELECT * FROM flights");
        assert_eq!(
            flights_by_airline(Some("KL")).unwrap(),
            "SELECT * FROM flights WHERE airline = 'KL'"
        );
    }

    #[test]
    fn test_when_true_keeps_all_clauses() {
        let result = QueryBuilder::new()
            .table("flights")
            .when(true, |q| {
                q.join("airports", "flights.origin", "airports.code")
                    .where_closure(|q| q.r#where("origin", "AMS").or_where("origin", "FRA"))
                    .order_by_asc("id")
                    .limit(10)
            })
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights JOIN airports ON flights.origin = airports.code WHERE (origin = 'AMS' OR origin = 'FRA') ORDER BY id ASC LIMIT 10"
        );
    }
}
