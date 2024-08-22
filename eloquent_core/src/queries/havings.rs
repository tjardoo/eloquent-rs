use crate::{Having, Operator, QueryBuilder};

impl QueryBuilder {
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
}
