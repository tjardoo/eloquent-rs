use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotInsertWithDifferentColumns;

impl PerformChecks for CannotInsertWithDifferentColumns {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.inserts.is_empty() {
            return Ok(());
        }

        let column_count = builder
            .inserts
            .first()
            .map(|insert| insert.values.len())
            .unwrap_or(0);

        let inconsistent_row = builder
            .inserts
            .iter()
            .find(|insert| insert.values.len() != column_count);

        if inconsistent_row.is_some() {
            return Err(EloquentError::InconsistentInsertColumns);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{eloquent_sql_row, error::EloquentError, QueryBuilder, ToSql};

    #[test]
    fn test_cannot_insert_with_different_columns() {
        let result = QueryBuilder::new()
            .table("users")
            .insert_many(vec![
                eloquent_sql_row! {
                    "name" => "Alice",
                    "email" => "alice@example.com",
                    "is_active" => true,
                },
                eloquent_sql_row! {
                    "name" => "Bob",
                    "email" => "bob@example.com",
                    "age" => 22,
                    "is_active" => false,
                },
            ])
            .to_sql();

        match result {
            Err(EloquentError::InconsistentInsertColumns) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
