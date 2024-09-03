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
        add_selects(builder.table.as_ref().unwrap(), &builder.selects, sql);
        add_joins(&builder.joins, sql);
        add_conditions(&builder.conditions, &builder.closures, sql, params)?;
        add_group_by(&builder.group_by, sql);
        add_havings(&builder.havings, sql)?;
        add_order_by(&builder.order_by, sql);
        add_limit_offset(&builder.limit, &builder.offset, sql);

        Ok(sql.to_string())
    }
}
