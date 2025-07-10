mod bind;
pub use bind::bind;

#[cfg(feature = "enable-raw")]
mod raw_sql;
#[cfg(feature = "enable-raw")]
pub use raw_sql::raw_sql;

#[cfg(test)]
#[cfg(not(feature = "disable-option-to-sql"))]
mod tests {
    #[test]
    fn test_option_none_to_sql() {
        use crate::ToSql;
        assert_eq!(Option::<bool>::None.to_sql().unwrap(), "NULL");
    }

    #[test]
    fn test_option_u32_to_sql() {
        use crate::ToSql;
        assert_eq!(Some(11).to_sql().unwrap(), "11");
    }

    #[test]
    fn test_option_str_to_sql() {
        use crate::ToSql;
        assert_eq!(Some("text").to_sql().unwrap(), "'text'");
    }
}
