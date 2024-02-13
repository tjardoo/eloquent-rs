use crate::{
    shared::{Closures, WhereClause, WhereOperator},
    Operator, Variable,
};

impl Closures {
    pub fn new(where_operator: WhereOperator) -> Self {
        Self {
            closures: Vec::new(),
            where_operator,
        }
    }

    fn create_where_clause(
        &mut self,
        column: &str,
        operator: Operator,
        value: Variable,
        where_operator: WhereOperator,
    ) -> &mut Self {
        self.closures.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator,
        });

        self
    }

    pub fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::And);

        self
    }

    pub fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Or);

        self
    }

    pub fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Not);

        self
    }

    pub fn where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::And);

        self
    }

    pub fn where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::And,
        );

        self
    }

    pub fn or_where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::Or);

        self
    }

    pub fn or_where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::Or,
        );

        self
    }
}
