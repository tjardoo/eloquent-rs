# Eloquent

This Rust library provides a fluent and type-safe query builder designed to simplify the construction of SQL queries for interacting with relational databases. With its expressive syntax, you can effortlessly build complex SQL queries using a chain of method calls that mirror SQL operations. The library supports a wide range of conditions such as WHERE, OR WHERE, IN, NOT IN, LIKE, and many more, while also allowing for nested conditions using closures.

Whether you’re filtering flights by departure airport, checking for null values, or creating complex nested conditions, this library ensures that your queries are easy to read, write, and maintain. Its design allows you to focus on the logic of your queries without worrying about the underlying SQL syntax, providing both power and flexibility in building database queries.

```rust
let result = Eloquent::query()
    .table("flights")
    .select("origin_airport")
    .select_avg("startup_time_in_minutes")
    .select_as("airports.city", "destination_city")
    .join(
        "airports",
        "flights.destination_airport",
        "airports.iata_code",
    )
    .r#where("origin_airport", "AMS")
    .where_not_in("flight_number", vec!["KL123", "KL456"])
    .where_not_null("gate_number")
    .where_closure(|q| {
        q.where_gte("flight_duration", 120)
            .or_where_like("airports.city", "%NY%")
    })
    .group_by(vec!["origin_airport", "airports.city"])
    .having_gt("AVG(startup_time_in_minutes)", 120)
    .order_by_asc("AVG(startup_time_in_minutes)")
    .limit(20);

println!("{}", result.sql().unwrap());
```
