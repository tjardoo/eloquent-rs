# Eloquent

Eloquent is a SQL query builder to easily build complex SQL queries in Rust. It is inspired by Laravel's Eloquent ORM and is designed to be simple and easy to use.

The query builder supports `select`, `insert`, `update`, `delete`, `where`, `join`, `group_by`, `having`, `order_by`, `limit`, `offset` and `to_sql` methods. See [Available Methods](./docs/available-methods.md) for more details. Also closures can be used to build complex where clauses.

## Usage

```ini
[dependencies]
eloquent = "1.0"
```

```rust
use eloquent_core::{Eloquent, Operator, Variable};

fn select_test_query_1() {
    let query = Eloquent::table("users")
        .r#where("created_at", Operator::GreaterThanOrEqual, Variable::String("2024-01-01".to_string()))
        .where_null("deleted_at")
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
        "SELECT * FROM users WHERE created_at >= `2024-01-01` AND deleted_at IS NULL AND (age >= 18 AND age < 25) OR (age >= 30 AND status IN (`pending`, `active`))"
    );
}
```
