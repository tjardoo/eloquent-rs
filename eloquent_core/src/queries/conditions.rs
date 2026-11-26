use crate::{
    Columnable, Condition, ConditionGroup, Logic, Operator, QueryBuilder, ToSql, WhereClause,
};

impl QueryBuilder {
    fn add_condition(
        mut self,
        field: &str,
        operator: Operator,
        logic: Logic,
        values: Vec<Box<dyn ToSql>>,
    ) -> Self {
        self.conditions.push(WhereClause::Condition(Condition::new(
            field, operator, logic, values,
        )));

        self
    }

    /// Add a where condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("origin", "AMS");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin = 'AMS'"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, SubqueryBuilder};
    ///
    /// let subquery = SubqueryBuilder::new()
    ///     .table("flights")
    ///     .select_max("duration_in_min", "max_duration_in_min");
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("id", subquery);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE id = (SELECT MAX(duration_in_min) AS max_duration_in_min FROM flights)"
    /// );
    /// ```
    pub fn r#where(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Equal, Logic::And, vec![Box::new(value)])
    }

    /// Add a where condition to the query (alias for `r#where`).
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_eq("origin", "AMS");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin = 'AMS'"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, SubqueryBuilder};
    ///
    /// let subquery = SubqueryBuilder::new()
    ///     .table("flights")
    ///     .select_max("duration_in_min", "max_duration_in_min");
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_eq("id", subquery);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE id = (SELECT MAX(duration_in_min) AS max_duration_in_min FROM flights)"
    /// );
    /// ```
    pub fn where_eq(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Equal, Logic::And, vec![Box::new(value)])
    }

    /// Add an OR where condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("origin", "AMS")
    ///     .or_where("destination", "FRA");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin = 'AMS' OR destination = 'FRA'"
    /// );
    /// ```
    pub fn or_where(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Equal, Logic::Or, vec![Box::new(value)])
    }

    /// Add a where not condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not("origin", "AMS");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin != 'AMS'"
    /// );
    /// ```
    pub fn where_not(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::NotEqual, Logic::And, vec![Box::new(value)])
    }

    /// Add an OR where not condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("origin", "AMS")
    ///     .or_where_not("destination", "AMS");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin = 'AMS' OR destination != 'AMS'"
    /// );
    /// ```
    pub fn or_where_not(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::NotEqual, Logic::Or, vec![Box::new(value)])
    }

    /// Add a where greater than condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_gt("flight_duration", 120);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration > 120"
    /// );
    /// ```
    pub fn where_gt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThan,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    /// Add an OR where greater than condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_gt("flight_duration", 120)
    ///     .or_where_gt("number_of_passengers", 200);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration > 120 OR number_of_passengers > 200"
    /// );
    /// ```
    pub fn or_where_gt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThan,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

    /// Add a where greater than or equal to condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_gte("flight_duration", 120);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration >= 120"
    /// );
    /// ```
    pub fn where_gte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThanOrEqual,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    /// Add an OR where greater than or equal to condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_gt("flight_duration", 120)
    ///     .or_where_gte("number_of_passengers", 200);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration > 120 OR number_of_passengers >= 200"
    /// );
    /// ```
    pub fn or_where_gte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThanOrEqual,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

    /// Add a where less than condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_lt("flight_duration", 120);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration < 120"
    /// );
    /// ```
    pub fn where_lt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::LessThan, Logic::And, vec![Box::new(value)])
    }

    /// Add an OR where less than condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_lt("flight_duration", 120)
    ///     .or_where_lt("number_of_passengers", 200);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration < 120 OR number_of_passengers < 200"
    /// );
    /// ```
    pub fn or_where_lt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::LessThan, Logic::Or, vec![Box::new(value)])
    }

    /// Add a where less than or equal to condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_lte("flight_duration", 120);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration <= 120"
    /// );
    /// ```
    pub fn where_lte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::LessThanOrEqual,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    /// Add an OR where less than or equal to condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_lte("flight_duration", 120)
    ///     .or_where_lte("number_of_passengers", 200);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration <= 120 OR number_of_passengers <= 200"
    /// );
    /// ```
    pub fn or_where_lte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::LessThanOrEqual,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

    /// Add a where between condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_between("flight_duration", 120, 180);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE flight_duration BETWEEN 120 AND 180"
    /// );
    /// ```
    pub fn where_between(
        self,
        field: &str,
        min: impl ToSql + 'static,
        max: impl ToSql + 'static,
    ) -> Self {
        self.add_condition(
            field,
            Operator::Between,
            Logic::And,
            vec![Box::new(min), Box::new(max)],
        )
    }

    /// Add an OR where between condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .r#where("origin", "AMS")
    ///     .or_where_between("flight_duration", 120, 180);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin = 'AMS' OR flight_duration BETWEEN 120 AND 180"
    /// );
    /// ```
    pub fn or_where_between(
        self,
        field: &str,
        min: impl ToSql + 'static,
        max: impl ToSql + 'static,
    ) -> Self {
        self.add_condition(
            field,
            Operator::Between,
            Logic::Or,
            vec![Box::new(min), Box::new(max)],
        )
    }

    /// Add a where LIKE condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_like("airplane_type", "Airbus%");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%'"
    /// );
    /// ```
    pub fn where_like(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Like, Logic::And, vec![Box::new(value)])
    }

    /// Add an OR where LIKE condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_like("airplane_type", "Airbus%")
    ///     .or_where_like("airplane_type", "Embraer%");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE airplane_type LIKE 'Airbus%' OR airplane_type LIKE 'Embraer%'"
    /// );
    /// ```
    pub fn or_where_like(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Like, Logic::Or, vec![Box::new(value)])
    }

    /// Add a where IN condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_in("origin_airport", vec!["AMS", "FRA"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA')"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, SubqueryBuilder};
    ///
    /// let subquery = SubqueryBuilder::new()
    ///     .table("flights")
    ///     .select("id")
    ///     .where_gt("duration_in_min", 120);
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_in("id", vec![subquery]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE id IN (SELECT id FROM flights WHERE duration_in_min > 120)"
    /// );
    /// ```
    pub fn where_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::In, Logic::And, boxed_values)
    }

    /// Add an OR where IN condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_in("origin_airport", vec!["AMS", "FRA"])
    ///     .or_where_in("destination_airport", vec!["AMS", "FRA"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin_airport IN ('AMS', 'FRA') OR destination_airport IN ('AMS', 'FRA')"
    /// );
    /// ```
    pub fn or_where_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::In, Logic::Or, boxed_values)
    }

    /// Add a where NOT IN condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_in("origin_airport", vec!["AMS", "FRA"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA')"
    /// );
    /// ```
    ///
    /// ```
    /// use eloquent_core::{QueryBuilder, SubqueryBuilder};
    ///
    /// let subquery = SubqueryBuilder::new()
    ///     .table("flights")
    ///     .select("id")
    ///     .where_gt("duration_in_min", 120);
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_in("id", vec![subquery]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE id NOT IN (SELECT id FROM flights WHERE duration_in_min > 120)"
    /// );
    /// ```
    pub fn where_not_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::NotIn, Logic::And, boxed_values)
    }

    /// Add an OR where NOT IN condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_in("origin_airport", vec!["AMS", "FRA"])
    ///     .or_where_not_in("destination_airport", vec!["AMS", "FRA"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE origin_airport NOT IN ('AMS', 'FRA') OR destination_airport NOT IN ('AMS', 'FRA')"
    /// );
    /// ```
    pub fn or_where_not_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::NotIn, Logic::Or, boxed_values)
    }

    /// Add a where NULL condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_null("departure_time")
    ///     .where_null(vec!["arrival_time", "gate_number"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE departure_time IS NULL AND arrival_time IS NULL AND gate_number IS NULL"
    /// );
    /// ```
    pub fn where_null<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self = self.add_condition(column, Operator::IsNull, Logic::And, vec![]);
        }

        self
    }

    /// Add an OR where NULL condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_null("departure_time")
    ///     .or_where_null("arrival_time");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE departure_time IS NULL OR arrival_time IS NULL"
    /// );
    /// ```
    pub fn or_where_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNull, Logic::Or, vec![])
    }

    /// Add a where NOT NULL condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_null("departure_time")
    ///     .where_not_null(vec!["arrival_time", "gate_number"]);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE departure_time IS NOT NULL AND arrival_time IS NOT NULL AND gate_number IS NOT NULL"
    /// );
    /// ```
    pub fn where_not_null<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self = self.add_condition(column, Operator::IsNotNull, Logic::And, vec![]);
        }

        self
    }

    /// Add an OR where NOT NULL condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_null("departure_time")
    ///     .or_where_not_null("arrival_time");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE departure_time IS NOT NULL OR arrival_time IS NOT NULL"
    /// );
    /// ```
    pub fn or_where_not_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNotNull, Logic::Or, vec![])
    }

    /// Add a where closure condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_closure(|query| query.r#where("origin", "AMS").or_where("origin", "FRA"));
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE (origin = 'AMS' OR origin = 'FRA')"
    /// );
    /// ```
    pub fn where_closure<F>(self, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.add_closure(Logic::And, closure)
    }

    /// Add an OR where closure condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_not_null("departure_time")
    ///     .or_where_closure(|query| query.r#where("origin", "AMS").r#where("destination", "FRA"));
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE departure_time IS NOT NULL OR (origin = 'AMS' AND destination = 'FRA')"
    /// );
    /// ```
    pub fn or_where_closure<F>(self, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.add_closure(Logic::Or, closure)
    }

    fn add_closure<F>(mut self, logic: Logic, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        let nested_builder = closure(QueryBuilder::new());

        self.conditions.push(WhereClause::Group(ConditionGroup {
            logic,
            clauses: nested_builder.conditions,
        }));

        self
    }

    /// Add a where date condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_date("departure_date", "2024-10-01");
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE DATE(departure_date) = '2024-10-01'"
    /// );
    /// ```
    pub fn where_date(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Date, Logic::And, vec![Box::new(value)])
    }

    /// Add a where year condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_year("departure_date", 2024);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE YEAR(departure_date) = 2024"
    /// );
    /// ```
    pub fn where_year(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Year, Logic::And, vec![Box::new(value)])
    }

    /// Add a where month condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_month("departure_date", 10);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE MONTH(departure_date) = 10"
    /// );
    /// ```
    pub fn where_month(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Month, Logic::And, vec![Box::new(value)])
    }

    /// Add a where day condition to the query.
    ///
    /// ```
    /// use eloquent_core::QueryBuilder;
    ///
    /// let result = QueryBuilder::new()
    ///     .table("flights")
    ///     .where_day("departure_date", 10);
    ///
    /// assert_eq!(
    ///     result.sql().unwrap(),
    ///     "SELECT * FROM flights WHERE DAY(departure_date) = 10"
    /// );
    /// ```
    pub fn where_day(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Day, Logic::And, vec![Box::new(value)])
    }
}

#[cfg(test)]
mod tests {
    use super::QueryBuilder;
    use crate::ToSql;

    #[test]
    fn test_where_eq_u32() {
        assert_eq!(
            QueryBuilder::new()
                .table("flights")
                .where_eq("id", 1)
                .to_sql()
                .unwrap()
                .as_str(),
            "SELECT * FROM flights WHERE id = 1"
        );
    }

    #[test]
    fn test_where_eq_str() {
        assert_eq!(
            QueryBuilder::new()
                .table("flights")
                .where_eq("id", "1")
                .to_sql()
                .unwrap()
                .as_str(),
            "SELECT * FROM flights WHERE id = '1'"
        );
    }

    #[test]
    fn test_nested_where_closures() {
        let result = QueryBuilder::new()
            .table("flights")
            .where_closure(|q| {
                q.r#where("a", 1).or_where_closure(|q| {
                    q.r#where("b", 2)
                        .where_closure(|q| q.r#where("c", 3).or_where("d", 4))
                })
            })
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE (a = 1 OR (b = 2 AND (c = 3 OR d = 4)))"
        );
    }

    #[test]
    fn test_empty_where_closure_is_skipped() {
        let result = QueryBuilder::new()
            .table("flights")
            .r#where("a", 1)
            .where_closure(|q| q)
            .where_closure(|q| q.when(false, |q| q.r#where("b", 2)))
            .sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights WHERE a = 1");

        let result = QueryBuilder::new()
            .table("flights")
            .where_closure(|q| q)
            .sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights");
    }

    #[test]
    fn test_where_in_empty_list() {
        let result = QueryBuilder::new()
            .table("flights")
            .where_in("id", Vec::<i32>::new())
            .sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights WHERE 0 = 1");

        let result = QueryBuilder::new()
            .table("flights")
            .where_not_in("id", Vec::<i32>::new())
            .sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights WHERE 1 = 1");
    }

    #[test]
    fn test_where_between_values_containing_comma() {
        let result = QueryBuilder::new()
            .table("flights")
            .where_between("name", "a, b", "c, d")
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE name BETWEEN 'a, b' AND 'c, d'"
        );
    }

    #[test]
    fn test_query_builder_as_value_is_parenthesized() {
        let result = QueryBuilder::new()
            .table("flights")
            .r#where(
                "airport_id",
                QueryBuilder::new()
                    .table("airports")
                    .select("id")
                    .r#where("code", "AMS"),
            )
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE airport_id = (SELECT id FROM airports WHERE code = 'AMS')"
        );

        let result = QueryBuilder::new()
            .table("flights")
            .where_in(
                "airport_id",
                vec![QueryBuilder::new().table("airports").select("id")],
            )
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE airport_id IN (SELECT id FROM airports)"
        );
    }

    #[test]
    fn test_closures_keep_their_position() {
        let result = QueryBuilder::new()
            .table("flights")
            .r#where("a", 1)
            .or_where_closure(|q| q.r#where("b", 2).r#where("c", 3))
            .r#where("d", 4)
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE a = 1 OR (b = 2 AND c = 3) AND d = 4"
        );

        let result = QueryBuilder::new()
            .table("flights")
            .where_closure(|q| q.r#where("a", 1).or_where("b", 2))
            .where_not_null("c")
            .where_closure(|q| {
                q.r#where("d", 4)
                    .or_where_closure(|q| q.r#where("e", 5))
                    .r#where("f", 6)
            })
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE (a = 1 OR b = 2) AND c IS NOT NULL AND (d = 4 OR (e = 5) AND f = 6)"
        );
    }
}
