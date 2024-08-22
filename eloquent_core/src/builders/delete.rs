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

        add_joins(builder, sql);
        add_conditions(builder, sql, params)?;
        add_havings(builder, sql)?;

        Ok(sql.to_string())
    }
}
