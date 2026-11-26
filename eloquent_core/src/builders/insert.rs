use crate::{compilers::inserts, error::EloquentError, SqlBuilder};

pub struct InsertBuilder;

impl SqlBuilder for InsertBuilder {
    fn build(builder: &crate::QueryBuilder, sql: &mut String) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        inserts::format(table, &builder.inserts, sql)?;

        Ok(sql.to_string())
    }
}
