use crate::{EloquentError, ToSql};

#[derive(Debug)]
pub struct RawSql(String);

/// Inject a raw SQL value into the query. This is an unsafe function as it can cause the query to misbehave.
///
/// Requires feature `enable-raw`.
///
/// ```
/// use eloquent_core::{QueryBuilder, raw_sql};
///
/// let result = QueryBuilder::new()
///     .table("flights")
///     .insert("hash", unsafe { raw_sql("'some_hash'::bytea") });
///
/// assert_eq!(
///     result.sql().unwrap(),
///     "INSERT INTO flights (hash) VALUES ('some_hash'::bytea)"
/// );
/// ```
//
pub unsafe fn raw_sql<S>(sql: S) -> RawSql
where
    S: Into<String>,
{
    RawSql(sql.into())
}

impl ToSql for RawSql {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.0.clone())
    }
}

impl std::fmt::Display for RawSql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SQL({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QueryBuilder;

    #[test]
    fn test_raw_sql() {
        assert_eq!(
            unsafe { raw_sql("'{}'::jsonb").to_sql() },
            Ok(String::from("'{}'::jsonb"))
        );
    }

    #[test]
    fn test_raw_query_builder_insert() {
        unsafe {
            let query = QueryBuilder::new()
                .table("flights")
                .insert("metadata", raw_sql(r#"'{"name":"value"}'::json"#));

            assert_eq!(
                query.to_sql().unwrap(),
                r#"INSERT INTO flights (metadata) VALUES ('{"name":"value"}'::json)"#
            );
        }
    }
}
