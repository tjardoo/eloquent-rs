use std::fmt::Display;

use crate::{ArrayVariable, Direction, Operator, Variable};

pub trait WhereClauseBuilder {
    fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self;
    fn where_null(&mut self, column: &str) -> &mut Self;
    fn where_not_null(&mut self, column: &str) -> &mut Self;
    fn or_where_null(&mut self, column: &str) -> &mut Self;
    fn or_where_not_null(&mut self, column: &str) -> &mut Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhereOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Count,
    Max,
    Min,
    Sum,
    Avg,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub column: String,
    pub operator: Operator,
    pub value: Variable,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: Operator,
    pub value: Variable,
    pub where_operator: WhereOperator,
}

#[derive(Debug, Clone)]
pub struct WhereClauses {
    pub clauses: Vec<Clause>,
    pub where_operator: WhereOperator,
}

#[derive(Debug, Clone)]
pub struct Join {
    pub table: String,
    pub left_hand: String,
    pub right_hand: String,
    pub r#type: JoinType,
}

#[derive(Debug, Clone)]
pub struct Closures {
    pub closures: Vec<WhereClause>,
    pub where_operator: WhereOperator,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::Like => write!(f, "LIKE"),
            Operator::NotLike => write!(f, "NOT LIKE"),
            Operator::In => write!(f, "IN"),
            Operator::NotIn => write!(f, "NOT IN"),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::String(s) => write!(f, "`{}`", s),
            Variable::Int(i) => write!(f, "{}", i),
            Variable::Bool(true) => write!(f, "{}", true),
            Variable::Bool(false) => write!(f, "{}", false),
            Variable::Null => write!(f, "IS NULL"),
            Variable::Array(a) => write!(
                f,
                "({})",
                a.iter()
                    .map(|v| match v {
                        ArrayVariable::String(s) => format!("`{}`", s),
                        ArrayVariable::Int(i) => format!("{}", i),
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Asc => write!(f, "ASC"),
            Direction::Desc => write!(f, "DESC"),
        }
    }
}

impl Display for WhereOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhereOperator::And => write!(f, "AND"),
            WhereOperator::Or => write!(f, "OR"),
            WhereOperator::Not => write!(f, "NOT"),
        }
    }
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::Inner => write!(f, "JOIN"),
            JoinType::Left => write!(f, "LEFT JOIN"),
            JoinType::Right => write!(f, "RIGHT JOIN"),
            JoinType::Full => write!(f, "FULL JOIN"),
        }
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionType::Count => write!(f, "COUNT"),
            FunctionType::Max => write!(f, "MAX"),
            FunctionType::Min => write!(f, "MIN"),
            FunctionType::Sum => write!(f, "SUM"),
            FunctionType::Avg => write!(f, "AVG"),
        }
    }
}

impl From<WhereClause> for Clause {
    fn from(where_clause: WhereClause) -> Self {
        Self {
            column: where_clause.column,
            operator: where_clause.operator,
            value: where_clause.value,
        }
    }
}
