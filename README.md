# Eloquent

[![tests](https://github.com/tjardoo/eloquent-rs/workflows/test/badge.svg?event=push)](https://github.com/tjardoo/eloquent-rs/actions)
[![crate.io](https://img.shields.io/crates/v/eloquent.svg)](https://crates.io/crates/eloquent)
[![docs](https://docs.rs/eloquent/badge.svg)](https://docs.rs/eloquent)

A Rust library for building queries in an eloquent way.

- [Usage](#usage)
- Examples
  - [Select query](#select-query)
  - [Insert query](#insert-query)
  - [Update query](#update-query)
  - [Delete query](#delete-query)

## Usage

```ini
[dependencies]
eloquent = "0.2"
```

### Select Query

```rust
use eloquent_core::{Direction, GenericVar};

let query = Eloquent::query()
    .table("flights")
    .select("id")
    .select("flight_number")
    .r#where("destination", GenericVar::Str("SIN".to_string()))
    .to_sql()
    .unwrap();

    assert_eq!(query, "SELECT `id`, `flight_number` FROM flights WHERE `destination` = \"SIN\";");
```

### Insert Query

```rust
use eloquent_core::{Direction, GenericVar, Clause};

let query = Eloquent::query()
    .insert("flights", vec![
        Clause {
            column: "id".to_string(),
            value: GenericVar::Int(1),
        },
        Clause {
            column: "flight_code".to_string(),
            value: GenericVar::Str("KL0803".to_string()),
        },
    ])
    .to_sql()
    .unwrap();

    assert_eq!(query, "INSERT INTO flights (`id`, `flight_code`) VALUES (1, \"KL0803\");");
```

### Update Query

```rust
use eloquent_core::{Direction, GenericVar, Clause};

let query = Eloquent::query()
    .update("flights", vec![
        Clause {
            column: "flight_code".to_string(),
            value: GenericVar::Str("KL0803".to_string()),
        },
        Clause {
            column: "destination".to_string(),
            value: GenericVar::Str("Bangkok".to_string()),
        },
    ])
    .r#where("id", GenericVar::Int(1))
    .to_sql()
    .unwrap();

    assert_eq!(query, "INSERT INTO flights (`id`, `flight_code`) VALUES (1, \"KL0803\") WHERE `id` = 1;");
```

### Delete Query

```rust
use eloquent_core::{Direction, GenericVar};

let query = Eloquent::query()
    .delete("flights")
    .r#where("id", GenericVar::Int(1))
    .to_sql()
    .unwrap();

assert_eq!(query, "DELETE FROM flights WHERE `id` = 1;");
```
