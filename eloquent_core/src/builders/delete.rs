use crate::{
    compiler::{add_conditions, add_havings, add_joins},
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
        sql.push_str("DELETE FROM ");
        sql.push_str(builder.table.as_ref().unwrap());

        add_joins(&builder.joins, sql);
        add_conditions(&builder.conditions, &builder.closures, sql, params)?;
        add_havings(&builder.havings, sql)?;

        Ok(sql.to_string())
    }
}
