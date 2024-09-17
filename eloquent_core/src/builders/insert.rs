use crate::{compilers::inserts, error::EloquentError, SqlBuilder};

pub struct InsertBuilder;

impl SqlBuilder for InsertBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn crate::ToSql + 'static)>>,
    ) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        inserts::format(table, &builder.inserts, sql, params);

        Ok(sql.to_string())
    }
}
