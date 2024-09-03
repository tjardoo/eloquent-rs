use crate::{Condition, Having, Logic, Operator, SubqueryBuilder};

impl SubqueryBuilder {
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

    pub fn having_between(mut self, column: &str, value_1: i64, value_2: i64) -> Self {
        self.havings.push(Having {
            conditions: vec![Condition {
                field: column.to_string(),
                operator: Operator::Between,
                logic: Logic::And,
                values: vec![Box::new(value_1), Box::new(value_2)],
            }],
        });

        self
    }

    fn add_having(mut self, column: &str, value: i64, operator: Operator) -> Self {
        self.havings.push(Having {
            conditions: vec![Condition {
                field: column.to_string(),
                operator,
                logic: Logic::And,
                values: vec![Box::new(value)],
            }],
        });

        self
    }
}
