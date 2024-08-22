use std::collections::HashSet;

use crate::{error::EloquentError, Logic, PerformChecks, QueryBuilder};

pub struct DuplicatedConditions;

impl PerformChecks for DuplicatedConditions {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        let mut seen = HashSet::new();

        for condition in &builder.conditions {
            if condition.logic == Logic::Or {
                continue;
            }

            if !seen.insert((&condition.field, condition.operator.to_string())) {
                return Err(EloquentError::DuplicatedConditions(condition.field.clone()));
            }
        }

        Ok(())
    }
}
