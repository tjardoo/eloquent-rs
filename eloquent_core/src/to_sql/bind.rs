use crate::{EloquentError, ToSql};

#[derive(Debug)]
pub struct Bind(u32);

/// Use a parameter binding for this value instead of a literal
#[cfg_attr(
    not(feature = "bind-use-question"),
    doc = r##"/// ```
/// use eloquent_core::{QueryBuilder, bind};
///
/// let result = QueryBuilder::new()
///     .table("flights")
///     .r#where("airline_id", bind(7));
///
/// assert_eq!(
///     result.sql().unwrap(),
///     "SELECT * FROM flights WHERE airline LIKE ?2"
/// );
/// ```
"##
)]
#[cfg_attr(
    not(feature = "bind-use-question"),
    doc = r##"
/// ```
/// use eloquent_core::{QueryBuilder, bind};
///
/// let result = QueryBuilder::new()
///     .table("flights")
///     .r#where("airline_id", bind(7));
///
/// assert_eq!(
///     result.sql().unwrap(),
///     "SELECT * FROM flights WHERE airline LIKE $2"
/// );
/// ```        
"##
)]
///
/// Notes:
/// Requires te `enable-bind` feature to be enabled
/// You can use the feature `bind-use-question` to control the use of
/// '$' vs '?' for formatting.
pub fn bind(index: u32) -> Bind {
    Bind(index)
}

impl ToSql for Bind {
    fn to_sql(&self) -> Result<String, EloquentError> {
        eprintln!("--> bind = {}", self.0);
        if cfg!(feature = "bind-use-question") {
            Ok(format!("?{}", self.0))
        } else {
            Ok(format!("${}", self.0))
        }
    }
}

impl std::fmt::Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bind({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QueryBuilder;

    #[test]
    #[cfg(not(feature = "bind-use-question"))]
    fn test_bind_dollar_sign() {
        assert_eq!(bind(2).to_sql(), Ok(String::from("$2")));
    }

    #[test]
    #[cfg(not(feature = "bind-use-question"))]
    fn test_bind_query_builder_delete() {
        let query = QueryBuilder::new()
            .table("flights")
            .delete()
            .r#where("id", bind(2));

        assert_eq!(query.to_sql().unwrap(), "DELETE FROM flights WHERE id = $2");
    }

    #[test]
    #[cfg(not(feature = "bind-use-question"))]
    fn test_bind_query_builder_insert() {
        let query = QueryBuilder::new().table("flights").insert("name", bind(2));

        assert_eq!(
            query.to_sql().unwrap(),
            "INSERT INTO flights (name) VALUES ($2)"
        );
    }

    #[test]
    #[cfg(feature = "bind-use-question")]
    fn test_bind_query_builder_insert() {
        let query = QueryBuilder::new().table("flights").insert("name", bind(4));

        assert_eq!(
            query.to_sql().unwrap(),
            "INSERT INTO flights (name) VALUES (?4)"
        );
    }

    #[test]
    #[cfg(feature = "bind-use-question")]
    fn test_bind_question_mark() {
        assert_eq!(bind(7).to_sql(), Ok(String::from("?7")));
    }
}
