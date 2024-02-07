# Eloquent

Eloquent is a simple and easy to use SQL query builder. The query builder supports `select`, `insert`, `update`, `delete`, `where`, `join`, `group_by`, `having`, `order_by`, `limit`, `offset` and `to_sql` methods.

## Usage

```ini
[dependencies]
eloquent = "1.0"
```

```rust
use eloquent_core::{Eloquent, Operator, Variable};

#[test]
fn select_test_query_1() {
    let mut builder = Eloquent::new();

    let query = builder
        .select(vec!["id".to_string(), "name".to_string()])
        .from("users".to_string())
        .r#where("id".to_string(), Operator::Equal, Variable::new(1))
        .to_sql();

    assert_eq!(query, "SELECT id, name FROM users WHERE id = 1");
}
```
