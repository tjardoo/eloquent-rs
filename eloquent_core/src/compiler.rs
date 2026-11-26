use crate::{
    builders::{
        delete::DeleteBuilder, insert::InsertBuilder, select::SelectBuilder, update::UpdateBuilder,
    },
    compilers::{conditions, group_by, havings, joins, limit, offset, order_by, selects},
    error::EloquentError,
    Action, QueryBuilder, SqlBuilder, SubqueryBuilder,
};

pub fn build_statement(builder: &QueryBuilder) -> Result<String, EloquentError> {
    if let Some(error) = &builder.error {
        return Err(error.clone());
    }

    if builder.enable_checks {
        builder.perform_checks()?;
    }

    let mut sql = String::new();

    sql = match builder.get_action() {
        Action::Select => SelectBuilder::build(builder, &mut sql)?,
        Action::Insert => InsertBuilder::build(builder, &mut sql)?,
        Action::Update => UpdateBuilder::build(builder, &mut sql)?,
        Action::Delete => DeleteBuilder::build(builder, &mut sql)?,
    };

    log::trace!(target: "eloquent", "{}", sql);

    Ok(sql)
}

pub fn build_substatement(builder: &SubqueryBuilder) -> Result<String, EloquentError> {
    if let Some(error) = &builder.error {
        return Err(error.clone());
    }

    let mut sql = String::new();
    sql.push('(');

    selects::format(
        builder.table.as_ref().unwrap(),
        &builder.selects,
        builder.distinct,
        &mut sql,
    );
    joins::format(&builder.joins, &mut sql);
    conditions::format(&builder.conditions, &None, &mut sql)?;
    group_by::format(&builder.group_by, &mut sql);
    havings::format(&builder.havings, &mut sql)?;
    order_by::format(&builder.order_by, &None, &mut sql);
    limit::format(&builder.limit, &None, &mut sql);
    offset::format(&builder.offset, &mut sql);

    sql.push(')');

    Ok(sql)
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder, SubqueryBuilder};

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

    #[test]
    fn test_where_value_containing_question_mark() {
        let result = QueryBuilder::new()
            .table("posts")
            .r#where("title", "Why?")
            .sql();

        assert_eq!(result.unwrap(), "SELECT * FROM posts WHERE title = 'Why?'");
    }

    #[test]
    fn test_question_mark_cannot_break_out_of_string_literal() {
        let result = QueryBuilder::new()
            .table("users")
            .r#where("name", " OR 1=1 -- ?")
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM users WHERE name = ' OR 1=1 -- ?'"
        );
    }

    #[test]
    fn test_values_containing_braces() {
        let result = QueryBuilder::new()
            .table("events")
            .r#where("payload", "{}")
            .r#where("id", 1)
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM events WHERE payload = '{}' AND id = 1"
        );

        let result = QueryBuilder::new()
            .table("events")
            .insert("payload", "{}")
            .insert("name", "a {} b")
            .sql();

        assert_eq!(
            result.unwrap(),
            "INSERT INTO events (payload, name) VALUES ('{}', 'a {} b')"
        );
    }

    #[test]
    fn test_update_with_question_marks_in_values() {
        let result = QueryBuilder::new()
            .table("posts")
            .update("title", "Why?")
            .r#where("slug", "why?")
            .sql();

        assert_eq!(
            result.unwrap(),
            "UPDATE posts SET title = 'Why?' WHERE slug = 'why?'"
        );
    }

    #[test]
    fn test_subquery_value_containing_question_mark() {
        let subquery = SubqueryBuilder::new()
            .table("users")
            .select("id")
            .r#where("name", "x?");

        let result = QueryBuilder::new()
            .table("posts")
            .where_in("user_id", vec![subquery])
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT * FROM posts WHERE user_id IN (SELECT id FROM users WHERE name = 'x?')"
        );
    }
}
