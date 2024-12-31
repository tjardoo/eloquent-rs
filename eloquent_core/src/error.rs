#[derive(Debug, PartialEq)]
pub enum EloquentError {
    MissingTable,
    DuplicatedColumnNames(String),
    DuplicatedConditions(String),
    HavingClauseWithoutAggregateFunction(String),
    GroupByWithNonSelectedOrAggregateFunction(String),
    OrderByWithNonSelectedOrAggregateFunction(String),
    MultipleCrudActions,
    MissingPlaceholders,
    CannotApplyClauseOnInsert(String),
    CannotApplyClauseOnUpdate(String),
    CannotApplyClauseOnDelete(String),
    CannotUseOffsetLimitWithPagination(String),
    InconsistentInsertColumns,
}

impl std::error::Error for EloquentError {}

impl std::fmt::Display for EloquentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EloquentError::MissingTable => write!(f, "Missing table"),
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
            EloquentError::MultipleCrudActions => write!(f, "Multiple CRUD actions"),
            EloquentError::MissingPlaceholders => write!(f, "Missing placeholders"),
            EloquentError::CannotApplyClauseOnInsert(clause) => {
                write!(f, "Cannot apply clause '{}' on INSERT", clause)
            }
            EloquentError::CannotApplyClauseOnUpdate(clause) => {
                write!(f, "Cannot apply clause '{}' on UPDATE", clause)
            }
            EloquentError::CannotApplyClauseOnDelete(clause) => {
                write!(f, "Cannot apply clause '{}' on DELETE", clause)
            }
            EloquentError::CannotUseOffsetLimitWithPagination(clause) => {
                write!(f, "Cannot use '{}' with PAGINATION", clause)
            }
            EloquentError::InconsistentInsertColumns => write!(
                f,
                "INSERT statement has inconsistent column counts across rows"
            ),
        }
    }
}
