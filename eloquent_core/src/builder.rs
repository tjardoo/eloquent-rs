use crate::{
    compiler::build_statement, error::EloquentError, Columnable, Condition, Function, Having, Join,
    JoinType, Logic, Operator, Order, QueryBuilder, Select, ToSql,
};

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            table: String::new(),
            selects: Vec::new(),
            conditions: Vec::new(),
            closures: Vec::new(),
            joins: Vec::new(),
            havings: Vec::new(),
            group_by: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
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

    pub fn select_as(mut self, column: &str, alias: &str) -> Self {
        self.selects.push(Select {
            function: None,
            column: column.to_string(),
            alias: Some(alias.to_string()),
        });

        self
    }

    pub fn select_count(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Count),
            column: column.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_min(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Min),
            column: column.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_max(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Max),
            column: column.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_avg(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Avg),
            column: column.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_sum(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Sum),
            column: column.to_string(),
            alias: None,
        });

        self
    }

    pub fn select_distinct(mut self, column: &str) -> Self {
        self.selects.push(Select {
            function: Some(Function::Distinct),
            column: column.to_string(),
            alias: None,
        });

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

    pub fn join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Inner)
    }

    pub fn left_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Left)
    }

    pub fn right_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Right)
    }

    pub fn full_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Full)
    }

    fn add_join(
        mut self,
        table: &str,
        left_hand: &str,
        right_hand: &str,
        join_type: JoinType,
    ) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            left_hand: left_hand.to_string(),
            join_type,
            right_hand: right_hand.to_string(),
        });

        self
    }

    pub fn having(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::Equal)
    }

    pub fn having_not(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::NotEqual)
    }

    pub fn having_gt(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::GreaterThan)
    }

    pub fn having_gte(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::GreaterThanOrEqual)
    }

    pub fn having_lt(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::LessThan)
    }

    pub fn having_lte(self, column: &str, value: i64) -> Self {
        self.add_having(column, value, Operator::LessThanOrEqual)
    }

    fn add_having(mut self, column: &str, value: i64, operator: Operator) -> Self {
        self.havings.push(Having {
            column: column.to_string(),
            value,
            operator,
        });

        self
    }

    pub fn group_by<T>(mut self, columns: T) -> Self
    where
        T: Columnable,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.group_by.push(column.to_string());
        }

        self
    }

    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(format!("{} {}", column, Order::Asc));

        self
    }

    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(format!("{} {}", column, Order::Desc));

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);

        self
    }

    pub fn sql(self) -> Result<String, EloquentError> {
        build_statement(self)
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
