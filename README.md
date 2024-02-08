# Eloquent

Eloquent is a simple and easy to use SQL query builder. The query builder supports `select`, `insert`, `update`, `delete`, `where`, `join`, `group_by`, `having`, `order_by`, `limit`, `offset` and `to_sql` methods.

## Usage

```ini
[dependencies]
eloquent = "1.0"
```

```rust
use eloquent_core::{Eloquent, Operator, Variable};

fn select_test_query_1() {
    let query = Eloquent::table("users")
        .select(vec!["id", "name"])
        .r#where("id", Operator::Equal, Variable::new(1))
        .to_sql();

    assert_eq!(query, "SELECT id, name FROM users WHERE id = 1");
}
```

```rust
use eloquent_core::{Eloquent, Operator, Variable};

fn select_test_query_1() {
    let query = Eloquent::table("users")
        .r#where("email", Operator::Equal, Variable::Null)
        .to_sql();

    assert_eq!(query, "SELECT * FROM users WHERE email IS NULL");
}
```
