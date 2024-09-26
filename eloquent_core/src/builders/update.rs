use crate::{
    compilers::{conditions, havings, joins, updates},
    error::EloquentError,
    SqlBuilder,
};

pub struct UpdateBuilder;

impl SqlBuilder for UpdateBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn crate::ToSql + 'static)>>,
    ) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        updates::format(table, &builder.updates, sql, params);
        joins::format(&builder.joins, sql);
        conditions::format(&builder.conditions, &builder.closures, &None, sql, params)?;
        havings::format(&builder.havings, sql)?;

        Ok(sql.to_string())
    }
}
