use crate::{
    checks::{
        cannot_apply_clause_on_delete::CannotApplyClauseOnDelete,
        cannot_apply_clause_on_insert::CannotApplyClauseOnInsert,
        cannot_apply_clause_on_update::CannotApplyClauseOnUpdate,
        duplicated_columns::DuplicatedColumns, duplicated_conditions::DuplicatedConditions,
        group_by_without_selected_or_aggregate_function::GroupByWithoutSelectedOrAggregateFunction,
        having_clause_without_aggregate_function::HavingClauseWithoutAggregateFunction,
        missing_table::MissingTable, multiple_crud_actions::MultipleCrudActions,
        order_by_without_selected_or_aggregate_function::OrderByWithoutSelectedOrAggregateFunction,
    },
    error::EloquentError,
    PerformChecks, QueryBuilder,
};

impl QueryBuilder {
    pub fn perform_checks(&self) -> Result<(), EloquentError> {
        MissingTable::check(self)?;
        DuplicatedColumns::check(self)?;
        DuplicatedConditions::check(self)?;
        GroupByWithoutSelectedOrAggregateFunction::check(self)?;
        HavingClauseWithoutAggregateFunction::check(self)?;
        OrderByWithoutSelectedOrAggregateFunction::check(self)?;
        MultipleCrudActions::check(self)?;
        CannotApplyClauseOnInsert::check(self)?;
        CannotApplyClauseOnUpdate::check(self)?;
        CannotApplyClauseOnDelete::check(self)?;

        Ok(())
    }
}
