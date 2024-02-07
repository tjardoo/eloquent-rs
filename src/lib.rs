//! # Eloquent
//!
//! Eloquent database query builder provides a convenient, fluent interface to create database queries.
//!

#[cfg(test)]
mod tests {
    use eloquent_core::{Eloquent, Operator, Variable};

    #[test]
    fn select_test_query_1() {
        let mut builder = Eloquent::new();

        let query: String = builder
            .select(vec!["id".to_string()])
            .from("users".to_string())
            .to_sql();

        assert_eq!(query, "SELECT id FROM users");
    }

    #[test]
    fn select_test_query_2() {
        let mut builder = Eloquent::new();

        let query = builder.from("users".to_string()).to_sql();

        assert_eq!(query, "SELECT * FROM users");
    }

    #[test]
    fn select_test_query_3() {
        let mut builder = Eloquent::new();

        let query = builder
            .from("users".to_string())
            .r#where("id".to_string(), Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn select_test_query_4() {
        let mut builder = Eloquent::new();

        let query = builder
            .select(vec!["id".to_string(), "name".to_string()])
            .select(vec!["email".to_string()])
            .from("users".to_string())
            .r#where("id".to_string(), Operator::Equal, Variable::Int(1))
            .r#where(
                "name".to_string(),
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
    fn insert_test_query_1() {
        let mut builder = Eloquent::new();

        let query = builder
            .insert(vec![("id".to_string(), Variable::Int(1))])
            .from("users".to_string())
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id) VALUES (1)");
    }

    #[test]
    fn insert_test_query_2() {
        let mut builder = Eloquent::new();

        let query = builder
            .insert(vec![("id".to_string(), Variable::Int(1))])
            .insert(vec![(
                "name".to_string(),
                Variable::String("John".to_string()),
            )])
            .from("users".to_string())
            .to_sql();

        assert_eq!(query, "INSERT INTO users (id, name) VALUES (1, `John`)");
    }

    #[test]
    fn update_test_query_1() {
        let mut builder = Eloquent::new();

        let query = builder
            .update(vec![(
                "name".to_string(),
                Variable::String("John".to_string()),
            )])
            .from("users".to_string())
            .r#where("id".to_string(), Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "UPDATE users SET name = `John` WHERE id = 1");
    }

    #[test]
    fn delete_test_query_1() {
        let mut builder = Eloquent::new();

        let query = builder
            .delete()
            .from("users".to_string())
            .r#where("id".to_string(), Operator::Equal, Variable::Int(1))
            .to_sql();

        assert_eq!(query, "DELETE FROM users WHERE id = 1");
    }
}
