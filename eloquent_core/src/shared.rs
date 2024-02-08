use crate::{Operator, Variable};

pub trait WhereClauseBuilder {
    fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn where_null(&mut self, column: &str) -> &mut Self;
    fn where_not_null(&mut self, column: &str) -> &mut Self;
    fn or_where_null(&mut self, column: &str) -> &mut Self;
    fn or_where_not_null(&mut self, column: &str) -> &mut Self;
}
