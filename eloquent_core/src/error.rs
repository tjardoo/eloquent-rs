#[derive(Debug)]
pub enum EloquentError {
    DuplicatedColumnNames(String),
    DuplicatedConditions(String),
    HavingClauseWithoutAggregateFunction(String),
    GroupByWithNonSelectedOrAggregateFunction(String),
    OrderByWithNonSelectedOrAggregateFunction(String),
}

impl std::error::Error for EloquentError {}

impl std::fmt::Display for EloquentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EloquentError::DuplicatedColumnNames(column) => {
                write!(f, "Duplicated column name '{}'", column)
            }
            EloquentError::DuplicatedConditions(column) => {
                write!(f, "Duplicated conditions '{}'", column)
            }
            EloquentError::HavingClauseWithoutAggregateFunction(column) => {
                write!(f, "HAVING clause without aggregate function '{}'", column)
            }
            EloquentError::GroupByWithNonSelectedOrAggregateFunction(column) => {
                write!(
                    f,
                    "GROUP BY without selected column or aggregate function '{}'",
                    column
                )
            }
            EloquentError::OrderByWithNonSelectedOrAggregateFunction(column) => {
                write!(
                    f,
                    "ORDER BY without selected or aggregate function '{}'",
                    column
                )
            }
        }
    }
}
