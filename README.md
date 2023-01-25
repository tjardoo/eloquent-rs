# Eloquent

[![Test Status](https://github.com/tjardoo/eloquent-rs/workflows/test/badge.svg?event=push)](https://github.com/tjardoo/eloquent-rs/actions)
[![Crate](https://img.shields.io/crates/v/eloquent.svg)](https://crates.io/crates/eloquent)
[![API](https://docs.rs/eloquent/badge.svg)](https://docs.rs/eloquent)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-lightgray.svg)](https://github.com/tjardoo/eloquent-rs#rust-version-requirements)

A Rust library for building queries in an eloquent way.

## Documentation

- [API reference (master branch)](https://github.com/tjardoo/eloquent-rs)
- [API reference (docs.rs)](https://docs.rs/eloquent/latest/eloquent)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
eloquent = "0.1.3"
```

### Select Query

```rs
use eloquent_core::{Direction, GenericVar};

let query = Eloquent::query()
    .table("flights".to_string())
    .select("id".to_string())
    .select("flight_number".to_string())
    .r#where("destination".to_string(), GenericVar::Str("SIN".to_string()))
    .to_sql()
    .unwrap();

    assert_eq!(query, "SELECT `id`, `flight_number` FROM flights WHERE `destination` = \"SIN\";");
```

### Insert Query

```rs
use eloquent_core::{Direction, GenericVar, Clause};

let query = Eloquent::query()
    .insert("flights".to_string(), vec![
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

```rs
use eloquent_core::{Direction, GenericVar, Clause};

let query = Eloquent::query()
    .update("flights".to_string(), vec![
        Clause {
            column: "flight_code".to_string(),
            value: GenericVar::Str("KL0803".to_string()),
        },
        Clause {
            column: "destination".to_string(),
            value: GenericVar::Str("Bangkok".to_string()),
        },
    ])
    .r#where("id".to_string(), GenericVar::Int(1))
    .to_sql()
    .unwrap();

    assert_eq!(query, "INSERT INTO flights (`id`, `flight_code`) VALUES (1, \"KL0803\") WHERE `id` = 1;");
```

### Delete Query

```rs
use eloquent_core::{Direction, GenericVar};

let query = Eloquent::query()
    .delete("flights".to_string())
    .r#where("id".to_string(), GenericVar::Int(1))
    .to_sql()
    .unwrap();

assert_eq!(query, "DELETE FROM flights WHERE `id` = 1;");
```
