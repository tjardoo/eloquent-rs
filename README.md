# Eloquent

> [!WARNING]
>
> This package is developed for learning purposes and is not intended for production use.

Eloquent is a SQL query builder to easily build complex SQL queries in Rust. It is inspired by Laravel's Query Builder and is designed to be simple and easy to use. This is not an ORM, in contrast to Laravel's Eloquent ORM. This libary is designed to be used with any SQL database and does not have any database specific features.

The query builder supports `select`, `insert`, `update`, `delete`, `where`, `join`, `group_by`, `having`, `order_by`, `limit`, `offset` and `to_sql` methods and support where clause closures.

See [Available Methods](./docs/available-methods.md) for more details.

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
