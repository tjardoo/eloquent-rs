//! # Eloquent
//!
//! Eloquent database query builder provides a convenient, fluent interface to create database queries.
//!

#[cfg(test)]
mod tests {
    use eloquent_core::{Direction, Eloquent, Operator, Variable};

    #[test]
    fn select_test_query_1() {
        let mut builder = Eloquent::new();

        let query: String = builder.select(vec!["id"]).table("users").to_sql();

        assert_eq!(query, "SELECT id FROM users");
    }

    #[test]
    fn select_test_query_2() {
        let mut builder = Eloquent::new();

        let query = builder.table("users").to_sql();

        assert_eq!(query, "SELECT * FROM users");
    }

    #[test]
    fn select_test_query_3() {
        let mut builder = Eloquent::new();

        let query = builder
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn select_test_query_4() {
        let mut builder = Eloquent::new();

        let query = builder
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
        let mut builder = Eloquent::new();

        let query = builder.table("users").limit(10).offset(20).to_sql();

        assert_eq!(query, "SELECT * FROM users LIMIT 10 OFFSET 20");
    }

    #[test]
    fn select_test_query_6() {
        let mut builder = Eloquent::new();

        let query = builder
            .table("users")
            .order_by("id", Direction::Desc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC");
    }

    #[test]
    fn select_test_query_7() {
        let mut builder = Eloquent::new();

        let query = builder
            .table("users")
            .order_by("id", Direction::Desc)
            .order_by("group_id", Direction::Asc)
            .to_sql();

        assert_eq!(query, "SELECT * FROM users ORDER BY id DESC, group_id ASC");
    }

    #[test]
    fn select_test_query_8() {
        let mut builder = Eloquent::new();

        let query = builder.table("users").group_by("group_id").to_sql();

        assert_eq!(query, "SELECT * FROM users GROUP BY group_id");
    }

    #[test]
    fn select_test_query_9() {
        let mut builder = Eloquent::new();

        let query = builder
            .table("users")
            .select_count("id")
            .select(vec!["country_id"])
            .group_by("country_id")
            .to_sql();

        assert_eq!(
            query,
            "SELECT COUNT(id), country_id FROM users GROUP BY country_id"
        );
    }

    #[test]
    fn select_test_query_10() {
        let mut builder = Eloquent::new();

        let query = builder
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
    fn insert_test_query_1() {
        let mut builder = Eloquent::new();

        let query = builder
            .insert(vec![("id", Variable::Int(1))])
            .table("users")
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id) VALUES (1)");
    }

    #[test]
    fn insert_test_query_2() {
        let mut builder = Eloquent::new();

        let query = builder
            .insert(vec![("id", Variable::Int(1))])
            .insert(vec![("name", Variable::String("John".to_string()))])
            .table("users")
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id, name) VALUES (1, `John`)");
    }

    #[test]
    fn update_test_query_1() {
        let mut builder = Eloquent::new();

        let query = builder
            .update(vec![("name", Variable::String("John".to_string()))])
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "UPDATE users SET name = `John` WHERE id = 1");
    }

    #[test]
    fn update_test_query_2() {
        let mut builder = Eloquent::new();

        let query = builder
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
        let mut builder = Eloquent::new();

        let query = builder
            .delete()
            .table("users")
            .r#where("id", Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "DELETE FROM users WHERE id = 1");
    }
}
