//! # Eloquent
//!
//! Eloquent database query builder provides a convenient, fluent interface to create database queries.
//!

#[cfg(test)]
mod tests {
    use eloquent_core::{Clause, Direction, Eloquent, Operator, Variable};

    #[test]
    fn select_test_query_1() {
        let query = Eloquent::query().select(vec!["id"]).table("users").to_sql();

        assert_eq!(query, "SELECT id FROM users");
    }

    #[test]
    fn select_test_query_2() {
        let query = Eloquent::query().table("users").to_sql();

        assert_eq!(query, "SELECT * FROM users");
    }

    #[test]
    fn select_test_query_3() {
        let query = Eloquent::query()
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn select_test_query_4() {
        let query = Eloquent::query()
            .select(vec!["id", "name"])
            .select(vec!["email"])
            .table("users")
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
        let query = Eloquent::query()
            .table("users")
            .limit(10)
            .offset(20)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users LIMIT 10 OFFSET 20");
    }

    #[test]
    fn select_test_query_6() {
        let query = Eloquent::query()
            .table("users")
            .order_by("id", Direction::Desc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC");
    }

    #[test]
    fn select_test_query_7() {
        let query = Eloquent::query()
            .table("users")
            .order_by("id", Direction::Desc)
            .order_by("group_id", Direction::Asc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC, group_id ASC");
    }

    #[test]
    fn select_test_query_8() {
        let query = Eloquent::query()
            .table("users")
            .group_by("group_id")
            .to_sql();

        assert_eq!(query, "SELECT * FROM users GROUP BY group_id");
    }

    #[test]
    fn select_test_query_9() {
        let query = Eloquent::query()
            .table("users")
            .select_count("id", "total_users")
            .select(vec!["country_id"])
            .group_by("country_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT COUNT(id) AS total_users, country_id FROM users GROUP BY country_id"
        );
    }

    #[test]
    fn select_test_query_10() {
        let query = Eloquent::query()
            .table("users")
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
        let query = Eloquent::query()
            .table("users")
            .join("purchases", "users.id", "purchase.user_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users JOIN purchases ON users.id = purchase.user_id"
        );
    }

    #[test]
    fn select_test_query_12() {
        let query = Eloquent::query()
            .table("users")
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
        let query = Eloquent::query()
            .table("users")
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
        let query = Eloquent::query()
            .table("users")
            .where_closure(vec![
                Clause {
                    column: "age".to_string(),
                    operator: Operator::GreaterThanOrEqual,
                    value: Variable::Int(18),
                },
                Clause {
                    column: "age".to_string(),
                    operator: Operator::LessThan,
                    value: Variable::Int(25),
                },
            ])
            .or_where_closure(vec![
                Clause {
                    column: "status".to_string(),
                    operator: Operator::Equal,
                    value: Variable::String("pending".to_string()),
                },
                Clause {
                    column: "status".to_string(),
                    operator: Operator::Equal,
                    value: Variable::String("active".to_string()),
                },
            ])
            .to_sql();

        assert_eq!(
            query,
            "SELECT * FROM users WHERE (age >= 18 AND age < 25) OR (status = `pending` AND status = `active`)"
        );
    }

    #[test]
    fn select_test_query_15() {
        let query = Eloquent::query()
            .table("users")
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
        let query = Eloquent::query()
            .insert(vec![("id", Variable::Int(1))])
            .table("users")
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id) VALUES (1)");
    }

    #[test]
    fn insert_test_query_2() {
        let query = Eloquent::query()
            .insert(vec![("id", Variable::Int(1))])
            .insert(vec![("name", Variable::String("John".to_string()))])
            .table("users")
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id, name) VALUES (1, `John`)");
    }

    #[test]
    fn update_test_query_1() {
        let query = Eloquent::query()
            .update(vec![("name", Variable::String("John".to_string()))])
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "UPDATE users SET name = `John` WHERE id = 1");
    }

    #[test]
    fn update_test_query_2() {
        let query = Eloquent::query()
            .update(vec![("name", Variable::String("John".to_string()))])
            .update(vec![(
                "email",
                Variable::String("john@example.com".to_string()),
            )])
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(
            query,
            "UPDATE users SET name = `John`, email = `john@example.com` WHERE id = 1"
        );
    }

    #[test]
    fn delete_test_query_1() {
        let query = Eloquent::query()
            .delete()
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "DELETE FROM users WHERE id = 1");
    }

    #[test]
    fn select_query_example() {
        let query = Eloquent::query()
            .select(vec!["user_id"])
            .select_count("order_id", "number_of_orders")
            .table("users")
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
        let query = Eloquent::query()
            .table("users")
            .r#where("email", Operator::Equal, Variable::Null)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE email IS NULL");
    }

    #[test]
    fn select_where_not_null_query() {
        let query = Eloquent::query()
            .table("users")
            .r#where("email", Operator::NotEqual, Variable::Null)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE email IS NOT NULL");
    }
}
