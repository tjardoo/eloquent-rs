use crate::{
    compilers::inserts::insert_columns, error::EloquentError, PerformChecks, QueryBuilder,
};

pub struct CannotInsertWithDifferentColumns;

impl PerformChecks for CannotInsertWithDifferentColumns {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.inserts.is_empty() {
            return Ok(());
        }

        insert_columns(&builder.inserts).map(|_| ())
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

    #[test]
    fn test_cannot_insert_rows_with_complementary_columns() {
        let result = QueryBuilder::new()
            .table("users")
            .insert_many(vec![
                eloquent_sql_row! { "name" => "Alice", "age" => 21 },
                eloquent_sql_row! { "name" => "Bob" },
                eloquent_sql_row! { "age" => 22 },
            ])
            .sql();

        assert_eq!(result, Err(EloquentError::InconsistentInsertColumns));
    }

    #[test]
    fn test_inconsistent_rows_error_without_validation() {
        let result = QueryBuilder::new()
            .table("users")
            .insert_many(vec![
                eloquent_sql_row! { "name" => "Alice", "age" => 21 },
                eloquent_sql_row! { "name" => "Bob" },
            ])
            .skip_validation()
            .sql();

        assert_eq!(result, Err(EloquentError::InconsistentInsertColumns));
    }

    #[test]
    fn test_insert_rows_with_columns_in_different_order() {
        let result = QueryBuilder::new()
            .table("users")
            .insert_many(vec![
                eloquent_sql_row! { "name" => "Alice", "age" => 21 },
                eloquent_sql_row! { "age" => 22, "name" => "Bob" },
            ])
            .sql();

        assert_eq!(
            result.unwrap(),
            "INSERT INTO users (name, age) VALUES ('Alice', 21), ('Bob', 22)"
        );
    }

    #[test]
    fn test_cannot_insert_duplicated_column_in_row() {
        let result = QueryBuilder::new()
            .table("users")
            .insert_many(vec![
                eloquent_sql_row! { "name" => "Alice", "name" => "Bob" },
            ])
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::DuplicatedColumnNames("name".to_string()))
        );
    }
}
