use crate::{Condition, Logic, Operator, ToSql};

pub struct QueryBuilder {
    table: String,
    conditions: Vec<Condition>,
    closures: Vec<(Logic, Vec<Condition>)>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            table: String::new(),
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

    pub fn or_where_closure<F>(mut self, closure: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        let mut nested_builder = QueryBuilder::new();

        nested_builder = closure(nested_builder);

        self.closures.push((Logic::Or, nested_builder.conditions));

        self
    }

    pub fn build_statement(self) -> String {
        let mut sql = format!("SELECT * FROM {}", self.table);

        let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

        if !self.conditions.is_empty() || !self.closures.is_empty() {
            sql.push_str(" WHERE ");

            let mut conditions_str = String::new();
            let mut first_condition = true;

            for (i, condition) in self.conditions.iter().enumerate() {
                if i > 0 {
                    conditions_str.push_str(match condition.logic {
                        Logic::And => " AND ",
                        Logic::Or => " OR ",
                    });
                }

                let condition_sql = match &condition.operator {
                    Operator::Equal => format!("{} = ?", condition.field),
                    Operator::NotEqual => format!("{} != ?", condition.field),
                    Operator::GreaterThan => format!("{} > ?", condition.field),
                    Operator::GreaterThanOrEqual => format!("{} >= ?", condition.field),
                    Operator::LessThan => format!("{} < ?", condition.field),
                    Operator::LessThanOrEqual => format!("{} <= ?", condition.field),
                    Operator::Like => format!("{} LIKE ?", condition.field),
                    Operator::In | Operator::NotIn => {
                        let placeholders = vec!["?"; condition.values.len()].join(", ");
                        if condition.operator == Operator::In {
                            format!("{}IN ({})", condition.field, placeholders)
                        } else {
                            format!("{}NOT IN ({})", condition.field, placeholders)
                        }
                    }
                    Operator::IsNull => format!("{} IS NULL", condition.field),
                    Operator::IsNotNull => format!("{} IS NOT NULL", condition.field),
                };

                conditions_str.push_str(&condition_sql);
                if !matches!(condition.operator, Operator::IsNull | Operator::IsNotNull) {
                    params.extend(condition.values.iter());
                }

                first_condition = false;
            }

            for (logic, closure) in self.closures.iter() {
                if !first_condition {
                    match logic {
                        Logic::And => conditions_str.push_str(" AND "),
                        Logic::Or => conditions_str.push_str(" OR "),
                    }
                }

                conditions_str.push('(');
                for (i, condition) in closure.iter().enumerate() {
                    if i > 0 {
                        conditions_str.push_str(match condition.logic {
                            Logic::And => " AND ",
                            Logic::Or => " OR ",
                        });
                    }

                    let condition_sql = match &condition.operator {
                        Operator::Equal => format!("{} = ?", condition.field),
                        Operator::NotEqual => format!("{} != ?", condition.field),
                        Operator::GreaterThan => format!("{} > ?", condition.field),
                        Operator::GreaterThanOrEqual => format!("{} >= ?", condition.field),
                        Operator::LessThan => format!("{} < ?", condition.field),
                        Operator::LessThanOrEqual => format!("{} <= ?", condition.field),
                        Operator::Like => format!("{} LIKE ?", condition.field),
                        Operator::In | Operator::NotIn => {
                            let placeholders = vec!["?"; condition.values.len()].join(", ");
                            if condition.operator == Operator::In {
                                format!("{} IN ({})", condition.field, placeholders)
                            } else {
                                format!("{} NOT IN ({})", condition.field, placeholders)
                            }
                        }
                        Operator::IsNull => format!("{} IS NULL", condition.field),
                        Operator::IsNotNull => format!("{} IS NOT NULL", condition.field),
                    };

                    conditions_str.push_str(&condition_sql);
                    if !matches!(condition.operator, Operator::IsNull | Operator::IsNotNull) {
                        params.extend(condition.values.iter());
                    }
                }
                conditions_str.push(')');
                first_condition = false;
            }

            sql.push_str(&conditions_str);
        }

        let formatted_sql = sql.replace('?', "{}");

        let formatted_sql = params
            .iter()
            .map(|p| p.as_ref().to_sql())
            .fold(formatted_sql, |acc, val| acc.replacen("{}", &val, 1));

        formatted_sql
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
