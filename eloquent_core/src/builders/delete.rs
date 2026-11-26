use crate::{
    compilers::{conditions, delete, havings, joins, limit, offset, order_by},
    error::EloquentError,
    SqlBuilder,
};

pub struct DeleteBuilder;

impl SqlBuilder for DeleteBuilder {
    fn build(builder: &crate::QueryBuilder, sql: &mut String) -> Result<String, EloquentError> {
        let table = builder.table.as_ref().ok_or(EloquentError::MissingTable)?;

        delete::format(table, sql);

        joins::format(&builder.joins, sql);
        conditions::format(&builder.conditions, &builder.paginate, sql)?;
        havings::format(&builder.havings, sql)?;
        order_by::format(&builder.order_by, &builder.paginate, sql);
        limit::format(&builder.limit, &builder.paginate, sql);
        offset::format(&builder.offset, sql);

        Ok(sql.to_string())
    }
}
