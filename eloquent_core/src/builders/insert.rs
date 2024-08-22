use crate::{compiler::add_inserts, error::EloquentError, SqlBuilder};

pub struct InsertBuilder;

impl SqlBuilder for InsertBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn crate::ToSql + 'static)>>,
    ) -> Result<String, EloquentError> {
        add_inserts(builder, sql, params);

        Ok(sql.to_string())
    }
}
