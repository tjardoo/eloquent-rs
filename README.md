# Eloquent

This Rust library provides a fluent and type-safe query builder designed to simplify the construction of SQL queries for interacting with relational databases. With its expressive syntax, you can effortlessly build complex SQL queries using a chain of method calls that mirror SQL operations. The library supports a wide range of conditions such as WHERE, OR WHERE, IN, NOT IN, LIKE, and many more, while also allowing for nested conditions using closures.

Whether you’re filtering flights by departure airport, checking for null values, or creating complex nested conditions, this library ensures that your queries are easy to read, write, and maintain. Its design allows you to focus on the logic of your queries without worrying about the underlying SQL syntax, providing both power and flexibility in building database queries.

```rust
let result = Eloquent::query()
    .table("flights")
    .select("origin_airport")
    .select_avg("startup_time_in_minutes", "startup_time_in_minutes_avg")
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
    .having_gt("startup_time_in_minutes_avg", 120)
    .order_by_asc("startup_time_in_minutes_avg")
    .limit(20);

println!("{}", result.pretty_sql().unwrap()); // or .sql() for unformatted SQL

// SELECT
//     origin_airport,
//     AVG(startup_time_in_minutes) AS startup_time_in_minutes_avg,
//     airports.city AS destination_city
// FROM
//     flights
//     JOIN airports ON flights.destination_airport = airports.iata_code
// WHERE
//     origin_airport = 'AMS'
//     AND flight_number NOT IN ('KL123', 'KL456')
//     AND gate_number IS NOT NULL
//     AND (
//         flight_duration >= 120
//         OR airports.city LIKE '%NY%'
//     )
// GROUP BY
//     origin_airport,
//     airports.city
// HAVING
//     startup_time_in_minutes_avg > 120
// ORDER BY
//     startup_time_in_minutes_avg ASC
// LIMIT
//     20
```
