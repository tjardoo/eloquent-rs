use crate::{Columnable, Condition, Logic, Operator, QueryBuilder, ToSql};

impl QueryBuilder {
    fn add_condition(
        mut self,
        field: &str,
        operator: Operator,
        logic: Logic,
        values: Vec<Box<dyn ToSql>>,
    ) -> Self {
        self.conditions
            .push(Condition::new(field, operator, logic, values));

        self
    }

    pub fn r#where(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Equal, Logic::And, vec![Box::new(value)])
    }

    pub fn or_where(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Equal, Logic::Or, vec![Box::new(value)])
    }

    pub fn where_not(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::NotEqual, Logic::And, vec![Box::new(value)])
    }

    pub fn or_where_not(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::NotEqual, Logic::Or, vec![Box::new(value)])
    }

    pub fn where_gt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThan,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    pub fn or_where_gt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThan,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

    pub fn where_gte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThanOrEqual,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    pub fn or_where_gte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::GreaterThanOrEqual,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

    pub fn where_lt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::LessThan, Logic::And, vec![Box::new(value)])
    }

    pub fn or_where_lt(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::LessThan, Logic::Or, vec![Box::new(value)])
    }

    pub fn where_lte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::LessThanOrEqual,
            Logic::And,
            vec![Box::new(value)],
        )
    }

    pub fn or_where_lte(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(
            field,
            Operator::LessThanOrEqual,
            Logic::Or,
            vec![Box::new(value)],
        )
    }

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

    pub fn where_like(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Like, Logic::And, vec![Box::new(value)])
    }

    pub fn or_where_like(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Like, Logic::Or, vec![Box::new(value)])
    }

    pub fn where_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::In, Logic::And, boxed_values)
    }

    pub fn or_where_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::In, Logic::Or, boxed_values)
    }

    pub fn where_not_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::NotIn, Logic::And, boxed_values)
    }

    pub fn or_where_not_in(self, field: &str, values: Vec<impl ToSql + 'static>) -> Self {
        let boxed_values = values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn ToSql>)
            .collect();

        self.add_condition(field, Operator::NotIn, Logic::Or, boxed_values)
    }

    pub fn where_null<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.conditions
                .push(Condition::new(column, Operator::IsNull, Logic::And, vec![]));
        }

        self
    }

    pub fn or_where_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNull, Logic::Or, vec![])
    }

    pub fn where_not_null<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.conditions.push(Condition::new(
                column,
                Operator::IsNotNull,
                Logic::And,
                vec![],
            ));
        }

        self
    }

    pub fn or_where_not_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNotNull, Logic::Or, vec![])
    }

    pub fn where_closure<F>(mut self, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        let mut nested_builder = QueryBuilder::new();

        nested_builder = closure(nested_builder);

        self.closures.push((Logic::And, nested_builder.conditions));

        self
    }

    pub fn or_where_closure<F>(mut self, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        let mut nested_builder = QueryBuilder::new();

        nested_builder = closure(nested_builder);

        self.closures.push((Logic::Or, nested_builder.conditions));

        self
    }

    pub fn where_date(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Date, Logic::And, vec![Box::new(value)])
    }

    pub fn where_year(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Year, Logic::And, vec![Box::new(value)])
    }

    pub fn where_month(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Month, Logic::And, vec![Box::new(value)])
    }

    pub fn where_day(self, field: &str, value: impl ToSql + 'static) -> Self {
        self.add_condition(field, Operator::Day, Logic::And, vec![Box::new(value)])
    }
}
