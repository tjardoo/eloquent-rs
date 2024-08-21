use crate::{
    checks::{
        duplicated_columns::DuplicatedColumns, duplicated_conditions::DuplicatedConditions,
        group_by_without_selected_or_aggregate_function::GroupByWithoutSelectedOrAggregateFunction,
        having_clause_without_aggregate_function::HavingClauseWithoutAggregateFunction,
        order_by_without_selected_or_aggregate_function::OrderByWithoutSelectedOrAggregateFunction,
    },
    error::EloquentError,
    PerformChecks, QueryBuilder,
};

impl QueryBuilder {
    pub fn perform_checks(&self) -> Result<(), EloquentError> {
        DuplicatedColumns::perform_checks(self)?;
        DuplicatedConditions::perform_checks(self)?;
        GroupByWithoutSelectedOrAggregateFunction::perform_checks(self)?;
        HavingClauseWithoutAggregateFunction::perform_checks(self)?;
        OrderByWithoutSelectedOrAggregateFunction::perform_checks(self)?;

        Ok(())
    }
}
