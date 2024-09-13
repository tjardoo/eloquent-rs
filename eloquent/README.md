# Eloquent

**Eloquent** is a fluent and type-safe query builder for Rust, designed to simplify SQL query construction. It allows you to build complex SQL queries using method chains that closely mirror SQL syntax, all while ensuring type safety and readability.

The library supports a wide range of SQL operations - `WHERE`, `JOIN`, `IN`, `NOT IN`, `LIKE`, and more—along with nested conditions via closures and subqueries. With features for filtering, grouping, and sorting, Eloquent provides the flexibility needed for constructing powerful, maintainable queries without sacrificing clarity or control over the SQL being generated.

```rust
use eloquent::Eloquent;

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
```

```sql
SELECT
    origin_airport,
    AVG(startup_time_in_minutes) AS startup_time_in_minutes_avg,
    airports.city AS destination_city
FROM
    flights
    JOIN airports ON flights.destination_airport = airports.iata_code
WHERE
    origin_airport = 'AMS'
    AND flight_number NOT IN ('KL123', 'KL456')
    AND gate_number IS NOT NULL
    AND (
        flight_duration >= 120
        OR airports.city LIKE '%NY%'
    )
GROUP BY
    origin_airport,
    airports.city
HAVING
    startup_time_in_minutes_avg > 120
ORDER BY
    startup_time_in_minutes_avg ASC
LIMIT
    20
```