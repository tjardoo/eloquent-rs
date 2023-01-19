pub use eloquent_core::Eloquent;

#[cfg(test)]
mod tests {
    use eloquent_core::{Direction, GenericVar, InsertClause, UpdateClause};

    use super::*;

    #[test]
    fn it_works_with_a_select_query() {
        let query = Eloquent::query()
            .table("flights".to_string())
            .select("id".to_string())
            .select("flight_number".to_string())
            .select("destination".to_string())
            .r#where("departure_code".to_string(), GenericVar::Str("AMS".to_string()))
            .r#where("destination".to_string(), GenericVar::Str("SIN".to_string()))
            .where_not("terminal_id".to_string(), GenericVar::Int(2))
            .r#where("is_active".to_string(), GenericVar::Bool(true))
            .group_by("destination".to_string())
            .order_by("flight_number".to_string(), Direction::Asc)
            .order_by("id".to_string(), Direction::Desc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT `id`, `flight_number`, `destination` FROM flights WHERE `departure_code` = \"AMS\" AND `destination` = \"SIN\" AND `terminal_id` != 2 AND `is_active` = 1 GROUP BY `destination` ORDER BY `flight_number` ASC, `id` DESC;");
    }

    #[test]
    fn it_works_with_an_insert_query() {
        let query = Eloquent::query()
            .insert("flights".to_string(), vec![
                InsertClause {
                    column: "id".to_string(),
                    value: GenericVar::Int(1),
                },
                InsertClause {
                    column: "flight_code".to_string(),
                    value: GenericVar::Str("KL0803".to_string()),
                },
                InsertClause {
                    column: "destination".to_string(),
                    value: GenericVar::Str("Bangkok".to_string()),
                }
            ])
            .to_sql()
            .unwrap();

        assert_eq!(query, "INSERT INTO flights (`id`, `flight_code`, `destination`) VALUES (1, \"KL0803\", \"Bangkok\");");
    }

    #[test]
    fn it_works_with_an_update_query() {
        let query = Eloquent::query()
            .update("flights".to_string(), vec![
                UpdateClause {
                    column: "flight_code".to_string(),
                    value: GenericVar::Str("KL0803".to_string()),
                },
                UpdateClause {
                    column: "destination".to_string(),
                    value: GenericVar::Str("Bangkok".to_string()),
                }
            ])
            .r#where("id".to_string(), GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "UPDATE flights SET `flight_code` = \"KL0803\", `destination` = \"Bangkok\" WHERE `id` = 1;");
    }
}
