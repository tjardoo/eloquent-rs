use crate::{
    shared::WhereClauseBuilder, Operator, Variable, WhereClause, WhereClosure, WhereOperator,
};

impl WhereClosure {
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
}

impl WhereClauseBuilder for WhereClosure {
    fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::And);

        self
    }

    fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Or);

        self
    }

    fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Not);

        self
    }

    fn where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::And);

        self
    }

    fn where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::And,
        );

        self
    }

    fn or_where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::Or);

        self
    }

    fn or_where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::Or,
        );

        self
    }
}
