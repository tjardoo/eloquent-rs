use crate::{
    compiler::{
        add_conditions, add_group_by, add_havings, add_joins, add_limit_offset, add_order_by,
        add_selects,
    },
    error::EloquentError,
    SqlBuilder,
};

pub struct SelectBuilder;

impl SqlBuilder for SelectBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<(dyn crate::ToSql + 'static)>>,
    ) -> Result<String, EloquentError> {
        add_selects(builder, sql);
        add_joins(builder, sql);
        add_conditions(builder, sql, params)?;
        add_group_by(builder, sql);
        add_havings(builder, sql)?;
        add_order_by(builder, sql);
        add_limit_offset(builder, sql);

        Ok(sql.to_string())
    }
}
