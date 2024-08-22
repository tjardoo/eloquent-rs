use crate::{
    compiler::{add_conditions, add_havings, add_joins, add_updates},
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
        add_updates(builder, sql, params);
        add_joins(builder, sql);
        add_conditions(builder, sql, params)?;
        add_havings(builder, sql)?;

        Ok(sql.to_string())
    }
}
