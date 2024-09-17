use crate::{
    compilers::{conditions, delete, havings, joins},
    error::EloquentError,
    SqlBuilder,
};

pub struct DeleteBuilder;

impl SqlBuilder for DeleteBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn crate::ToSql + 'static)>>,
    ) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        delete::format(table, sql);

        joins::format(&builder.joins, sql);
        conditions::format(&builder.conditions, &builder.closures, sql, params)?;
        havings::format(&builder.havings, sql)?;

        Ok(sql.to_string())
    }
}
