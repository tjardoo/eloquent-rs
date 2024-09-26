use crate::{checks::*, error::EloquentError, PerformChecks, QueryBuilder};

impl QueryBuilder {
    pub(crate) fn perform_checks(&self) -> Result<(), EloquentError> {
        missing_table::MissingTable::check(self)?;
        multiple_crud_actions::MultipleCrudActions::check(self)?;
        duplicated_columns::DuplicatedColumns::check(self)?;
        duplicated_conditions::DuplicatedConditions::check(self)?;
        group_by_without_selected_or_aggregate_function::GroupByWithoutSelectedOrAggregateFunction::check(self)?;
        having_clause_without_aggregate_function::HavingClauseWithoutAggregateFunction::check(
            self,
        )?;
        order_by_without_selected_or_aggregate_function::OrderByWithoutSelectedOrAggregateFunction::check(self)?;
        cannot_apply_clause_on_insert::CannotApplyClauseOnInsert::check(self)?;
        cannot_apply_clause_on_update::CannotApplyClauseOnUpdate::check(self)?;
        cannot_apply_clause_on_delete::CannotApplyClauseOnDelete::check(self)?;
        cannot_use_offset_limit_with_pagination::CannotUseOffsetLimitWithPagination::check(self)?;

        Ok(())
    }
}
