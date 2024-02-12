//! # Eloquent
//!
//! Eloquent database query builder provides a convenient, fluent interface to create database queries.
//!
mod prelude;
pub use prelude::*;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use eloquent_core::{ArrayVariable, Direction, Eloquent, Operator, Variable};
    use std::vec;

    #[test]
    fn select_test_query_1_all_columns() {
        let query = Eloquent::table("users").to_sql();

        assert_eq!(query, "SELECT * FROM users");
    }

    #[test]
    fn select_test_query_1_single_columns() {
        let query = Eloquent::table("users")
            .select("id")
            .select("name")
            .to_sql();

        assert_eq!(query, "SELECT id, name FROM users");
    }

    #[test]
    fn select_test_query_1_multiple_columns() {
        let query = Eloquent::table("users").select(vec!["id", "name"]).to_sql();

        assert_eq!(query, "SELECT id, name FROM users");
    }

    #[test]
    fn select_test_query_2() {
        let query = Eloquent::table("users").to_sql();

        assert_eq!(query, "SELECT * FROM users");
    }

    #[test]
    fn select_test_query_3() {
        let query = Eloquent::table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn select_test_query_4() {
        let query = Eloquent::table("users")
            .select(vec!["id", "name"])
            .select("email")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .r#where(
                "name",
                Operator::Equal,
                Variable::String("John".to_string()),
            )
            .to_sql();

        assert_eq!(
            query,
            "SELECT id, name, email FROM users WHERE id = 1 AND name = `John`"
        );
    }

    #[test]
    fn select_test_query_5() {
        let query = Eloquent::table("users").limit(10).offset(20).to_sql();

        assert_eq!(query, "SELECT * FROM users LIMIT 10 OFFSET 20");
    }

    #[test]
    fn select_test_query_6() {
        let query = Eloquent::table("users")
            .order_by("id", Direction::Desc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC");
    }

    #[test]
    fn select_test_query_7() {
        let query = Eloquent::table("users")
            .order_by("id", Direction::Desc)
            .order_by("group_id", Direction::Asc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC, group_id ASC");
    }

    #[test]
    fn select_test_query_8() {
        let query = Eloquent::table("users").group_by("group_id").to_sql();

        assert_eq!(query, "SELECT * FROM users GROUP BY group_id");
    }

    #[test]
    fn select_test_query_9() {
        let query = Eloquent::table("users")
            .select_count("id", "total_users")
            .select("country_id")
            .group_by("country_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT COUNT(id) AS total_users, country_id FROM users GROUP BY country_id"
        );
    }

    #[test]
    fn select_test_query_10() {
        let query = Eloquent::table("users")
            .having(
                "total_purchases",
                Operator::GreaterThanOrEqual,
                Variable::Int(100),
            )
            .to_sql();

        assert_eq!(query, "SELECT * FROM users HAVING total_purchases >= 100");
    }

    #[test]
    fn select_test_query_11() {
        let query = Eloquent::table("users")
            .join("purchases", "users.id", "purchase.user_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users JOIN purchases ON users.id = purchase.user_id"
        );
    }

    #[test]
    fn select_test_query_12() {
        let query = Eloquent::table("users")
            .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
            .r#where("age", Operator::LessThan, Variable::Int(25))
            .or_where("age", Operator::GreaterThanOrEqual, Variable::Int(30))
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE age >= 18 AND age < 25 OR age >= 30"
        );
    }

    #[test]
    fn select_test_query_13() {
        let query = Eloquent::table("users")
            .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
            .r#where("age", Operator::LessThan, Variable::Int(25))
            .or_where(
                "status",
                Operator::Equal,
                Variable::String("pending".to_string()),
            )
            .or_where(
                "status",
                Operator::Equal,
                Variable::String("active".to_string()),
            )
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE age >= 18 AND age < 25 OR status = `pending` OR status = `active`"
        );
    }

    #[test]
    fn select_test_query_14() {
        let query = Eloquent::table("users")
            .where_closure(|closure| {
                closure
                    .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
                    .r#where("age", Operator::LessThan, Variable::Int(25));
            })
            .where_closure(|closure| {
                closure
                    .r#where(
                        "status",
                        Operator::Equal,
                        Variable::String("pending".to_string()),
                    )
                    .or_where(
                        "status",
                        Operator::Equal,
                        Variable::String("active".to_string()),
                    );
            })
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE (age >= 18 AND age < 25) AND (status = `pending` OR status = `active`)"
        );
    }

    #[test]
    fn select_test_query_15() {
        let query = Eloquent::table("users")
            .left_join("purchases", "users.id", "purchase.user_id")
            .right_join("orders", "users.id", "orders.user_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users LEFT JOIN purchases ON users.id = purchase.user_id RIGHT JOIN orders ON users.id = orders.user_id"
        );
    }

    #[test]
    fn insert_test_query_1() {
        let query = Eloquent::table("users")
            .insert(vec![("id", Variable::Int(1))])
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id) VALUES (1)");
    }

    #[test]
    fn insert_test_query_2() {
        let query = Eloquent::table("users")
            .insert(vec![("id", Variable::Int(1))])
            .insert(vec![("name", Variable::String("John".to_string()))])
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id, name) VALUES (1, `John`)");
    }

    #[test]
    fn update_test_query_1() {
        let query = Eloquent::table("users")
            .update(vec![("name", Variable::String("John".to_string()))])
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "UPDATE users SET name = `John` WHERE id = 1");
    }

    #[test]
    fn update_test_query_2() {
        let query = Eloquent::table("users")
            .update(vec![("name", Variable::String("John".to_string()))])
            .update(vec![(
                "email",
                Variable::String("john@example.com".to_string()),
            )])
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(
            query,
            "UPDATE users SET name = `John`, email = `john@example.com` WHERE id = 1"
        );
    }

    #[test]
    fn delete_test_query_1() {
        let query = Eloquent::table("users")
            .delete()
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "DELETE FROM users WHERE id = 1");
    }

    #[test]
    fn select_query_example() {
        let query = Eloquent::table("users")
            .select("user_id")
            .select_count("order_id", "number_of_orders")
            .join("orders", "users.user_id", "orders.user_id")
            .r#where("age", Operator::GreaterThan, Variable::Int(18))
            .group_by("user_id")
            .having("total_orders", Operator::GreaterThan, Variable::Int(5))
            .order_by("total_orders", Direction::Desc)
            .limit(10)
            .offset(0)
            .to_sql();

        assert_eq!(query, "SELECT user_id, COUNT(order_id) AS number_of_orders FROM users JOIN orders ON users.user_id = orders.user_id WHERE age > 18 GROUP BY user_id HAVING total_orders > 5 ORDER BY total_orders DESC LIMIT 10 OFFSET 0");
    }

    #[test]
    fn select_where_null_query() {
        let query = Eloquent::table("users")
            .r#where("email", Operator::Equal, Variable::Null)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE email IS NULL");
    }

    #[test]
    fn select_where_not_null_query() {
        let query = Eloquent::table("users")
            .r#where("email", Operator::NotEqual, Variable::Null)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE email IS NOT NULL");
    }

    #[test]
    fn select_where_in_query() {
        let query = Eloquent::table("users")
            .r#where(
                "country_id",
                Operator::In,
                Variable::Array(vec![
                    ArrayVariable::String("NL".to_string()),
                    ArrayVariable::String("DE".to_string()),
                ]),
            )
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE country_id IN (`NL`, `DE`)"
        );
    }

    #[test]
    fn select_where_not_in_query() {
        let query = Eloquent::table("users")
            .r#where(
                "continent_id",
                Operator::NotIn,
                Variable::Array(vec![
                    ArrayVariable::String("AF".to_string()),
                    ArrayVariable::String("SA".to_string()),
                ]),
            )
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE continent_id NOT IN (`AF`, `SA`)"
        );
    }

    #[test]
    fn select_where_null_function_query() {
        let query = Eloquent::table("users")
            .where_null("email")
            .or_where_null("phone")
            .where_not_null("activated_at")
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE email IS NULL OR phone IS NULL AND activated_at IS NOT NULL"
        );
    }

    #[test]
    fn select_where_null_closure_query() {
        let query = Eloquent::table("users")
            .where_closure(|closure| {
                closure
                    .r#where("email", Operator::Equal, Variable::Null)
                    .or_where("phone", Operator::Equal, Variable::Null);
            })
            .where_not_null("activated_at")
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE activated_at IS NOT NULL AND (email IS NULL OR phone IS NULL)"
        );
    }

    #[test]
    fn select_test_query_16() {
        let query = Eloquent::table("users")
            .where_closure(|closure| {
                closure
                    .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
                    .r#where("age", Operator::LessThan, Variable::Int(25));
            })
            .or_where_closure(|closure| {
                closure
                    .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(30))
                    .r#where(
                        "status",
                        Operator::In,
                        Variable::Array(vec![
                            ArrayVariable::String("pending".to_string()),
                            ArrayVariable::String("active".to_string()),
                        ]),
                    );
            })
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE (age >= 18 AND age < 25) OR (age >= 30 AND status IN (`pending`, `active`))"
        );
    }
}
