use crate::{
    builders::{
        delete::DeleteBuilder, insert::InsertBuilder, select::SelectBuilder, update::UpdateBuilder,
    },
    compilers::{conditions, group_by, havings, joins, limit, offset, order_by, selects},
    error::EloquentError,
    Action, Condition, Logic, QueryBuilder, SqlBuilder, SubqueryBuilder, ToSql,
};

pub fn build_statement(builder: &QueryBuilder) -> Result<String, EloquentError> {
    if builder.enable_checks {
        builder.perform_checks()?;
    }

    let mut sql = String::new();
    let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

    sql = match builder.get_action() {
        Action::Select => SelectBuilder::build(builder, &mut sql, &mut params)?,
        Action::Insert => InsertBuilder::build(builder, &mut sql, &mut params)?,
        Action::Update => UpdateBuilder::build(builder, &mut sql, &mut params)?,
        Action::Delete => DeleteBuilder::build(builder, &mut sql, &mut params)?,
    };

    let formatted_sql = sql.replace('?', "{}");

    let formatted_sql = params
        .iter()
        .map(|p| p.as_ref().to_sql())
        .fold(formatted_sql, |acc, val| {
            acc.replacen("{}", &val.unwrap(), 1)
        });

    if formatted_sql.contains("{}") {
        return Err(EloquentError::MissingPlaceholders);
    }

    log::trace!(target: "eloquent", "{}", formatted_sql);

    Ok(formatted_sql)
}

pub fn build_substatement(builder: &SubqueryBuilder) -> Result<String, EloquentError> {
    let mut sql = String::new();
    sql.push('(');

    let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

    let closures: Vec<(Logic, Vec<Condition>)> = Vec::new();

    selects::format(builder.table.as_ref().unwrap(), &builder.selects, &mut sql);
    joins::format(&builder.joins, &mut sql);
    conditions::format(&builder.conditions, &closures, &None, &mut sql, &mut params)?;
    group_by::format(&builder.group_by, &mut sql);
    havings::format(&builder.havings, &mut sql)?;
    order_by::format(&builder.order_by, &mut sql);
    limit::format(&builder.limit, &mut sql);
    offset::format(&builder.offset, &mut sql);

    sql.push(')');

    let formatted_sql = sql.replace('?', "{}");

    let formatted_sql = params
        .iter()
        .map(|p| p.as_ref().to_sql())
        .fold(formatted_sql, |acc, val| {
            acc.replacen("{}", &val.unwrap(), 1)
        });

    if formatted_sql.contains("{}") {
        return Err(EloquentError::MissingPlaceholders);
    }

    Ok(formatted_sql)
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_missing_placeholder() {
        let result = QueryBuilder::new()
            .select_raw(
                "flight_duration * ? as delay_in_min, delay_in_min * ? as delay_in_hr",
                vec![5],
            )
            .table("flights")
            .sql();

        match result {
            Err(EloquentError::MissingPlaceholders) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_skip_validation() {
        let result = QueryBuilder::new().table("flights").skip_validation().sql();

        assert_eq!(result.unwrap(), "SELECT * FROM flights");
    }

    #[test]
    fn test_escapes_single_quotes() {
        let result = QueryBuilder::new()
            .table("flights")
            .r#where("origin_airport", "'N ABC 'S")
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM flights WHERE origin_airport = '\'\'N ABC \'\'S'"
        );
    }
}
