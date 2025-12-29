use crate::{
    compilers::{conditions, group_by, havings, joins, limit, offset, order_by, selects},
    error::EloquentError,
    SqlBuilder,
};

pub struct SelectBuilder;

impl SqlBuilder for SelectBuilder {
    fn build<'a>(
        builder: &'a crate::QueryBuilder,
        sql: &mut String,
        params: &mut Vec<&'a Box<dyn crate::ToSql + 'static>>,
    ) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        selects::format(table, &builder.selects, sql);
        joins::format(&builder.joins, sql);
        conditions::format(
            &builder.conditions,
            &builder.closures,
            &builder.paginate,
            sql,
            params,
        )?;
        group_by::format(&builder.group_by, sql);
        havings::format(&builder.havings, sql)?;
        order_by::format(&builder.order_by, sql);
        limit::format(&builder.limit, &builder.paginate, sql);
        offset::format(&builder.offset, sql);

        Ok(sql.to_string())
    }
}
