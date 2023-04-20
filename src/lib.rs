//! # Eloquent
//!
//! Eloquent database query builder provides a convenient, fluent interface to create database queries.
//!
//! # Quick Start
//!
//! ```rust
//! use eloquent_core::Eloquent;
//!
//! let query = Eloquent::query()
//!     .table("users")
//!     .select("first_name")
//!     .to_sql()
//!     .unwrap();
//!
//! assert_eq!(query, "SELECT `first_name` FROM users;");
//! ```
//!
//! # Support
//!
//! - [X] SELECT
//! - [X] WHERE
//! - [X] WHERE NOT
//! - [X] WHERE IS NULL
//! - [X] WHERE IS NOT NULL
//! - [X] INSERT
//! - [X] UPDATE
//! - [X] DELETE
//! - [X] ORDER BY
//! - [X] GROUP BY
//!

pub use eloquent_core::Eloquent;

#[cfg(test)]
mod tests {
    use eloquent_core::{Direction, GenericVar, Clause};

    use super::*;

    #[test]
    fn it_works_with_a_select_query() {
        let query = Eloquent::query()
            .table("flights")
            .select("id")
            .select("flight_number")
            .select("destination")
            .r#where("departure_code", GenericVar::Str("AMS"))
            .r#where("destination", GenericVar::Str("SIN"))
            .where_not("terminal_id", GenericVar::Int(2))
            .r#where("is_active", GenericVar::Bool(true))
            .group_by("destination")
            .order_by("flight_number", Direction::Asc)
            .order_by("id", Direction::Desc)
            .to_sql()
            .unwrap();

        assert_eq!(query, "SELECT `id`, `flight_number`, `destination` FROM flights WHERE `departure_code` = \"AMS\" AND `destination` = \"SIN\" AND `terminal_id` != 2 AND `is_active` = 1 GROUP BY `destination` ORDER BY `flight_number` ASC, `id` DESC;");
    }

    #[test]
    fn it_works_with_an_insert_query() {
        let query = Eloquent::query()
            .insert("flights", vec![
                Clause {
                    column: "id".to_string(),
                    value: GenericVar::Int(1),
                },
                Clause {
                    column: "flight_code".to_string(),
                    value: GenericVar::Str("KL0803"),
                },
                Clause {
                    column: "destination".to_string(),
                    value: GenericVar::Str("Bangkok"),
                }
            ])
            .to_sql()
            .unwrap();

        assert_eq!(query, "INSERT INTO flights (`id`, `flight_code`, `destination`) VALUES (1, \"KL0803\", \"Bangkok\");");
    }

    #[test]
    fn it_works_with_an_update_query() {
        let query = Eloquent::query()
            .update("flights", vec![
                Clause {
                    column: "flight_code".to_string(),
                    value: GenericVar::Str("KL0803"),
                },
                Clause {
                    column: "destination".to_string(),
                    value: GenericVar::Str("Bangkok"),
                }
            ])
            .r#where("id", GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "UPDATE flights SET `flight_code` = \"KL0803\", `destination` = \"Bangkok\" WHERE `id` = 1;");
    }

    #[test]
    fn it_works_with_a_delete_query() {
        let query = Eloquent::query()
            .delete("flights")
            .r#where("id", GenericVar::Int(1))
            .to_sql()
            .unwrap();

        assert_eq!(query, "DELETE FROM flights WHERE `id` = 1;");
    }
}
