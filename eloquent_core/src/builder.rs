use crate::{compiler::build_statement, Columnable, Condition, Logic, Operator, ToSql};

pub struct QueryBuilder {
    pub table: String,
    pub selects: Vec<String>,
    pub conditions: Vec<Condition>,
    pub closures: Vec<(Logic, Vec<Condition>)>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            table: String::new(),
            selects: Vec::new(),
            conditions: Vec::new(),
            closures: Vec::new(),
        }
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = table.to_string();

        self
    }

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

    pub fn where_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNull, Logic::And, vec![])
    }

    pub fn or_where_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNull, Logic::Or, vec![])
    }

    pub fn where_not_null(self, field: &str) -> Self {
        self.add_condition(field, Operator::IsNotNull, Logic::And, vec![])
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

    pub fn select<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.selects.push(column.to_string());
        }

        self
    }

    pub fn select_as(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(format!("{} AS {}", column, alias));

        self
    }

    pub fn select_count(mut self, column: &str) -> Self {
        self.selects.push(format!("COUNT({})", column));

        self
    }

    pub fn select_min(mut self, column: &str) -> Self {
        self.selects.push(format!("MIN({})", column));

        self
    }

    pub fn select_max(mut self, column: &str) -> Self {
        self.selects.push(format!("MAX({})", column));

        self
    }

    pub fn select_avg(mut self, column: &str) -> Self {
        self.selects.push(format!("AVG({})", column));

        self
    }

    pub fn select_sum(mut self, column: &str) -> Self {
        self.selects.push(format!("SUM({})", column));

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

    pub fn build(self) -> String {
        build_statement(self)
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
