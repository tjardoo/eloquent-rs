//! # Eloquent
//!
//! **Eloquent** is a fluent, type-safe query builder for Rust, designed to make SQL query construction intuitive. It provides an expressive API, allowing developers to craft complex SQL queries through method chaining.
//!
//! ## Features
//!
//! - Fluent API for building SQL queries.
//! - Type-safe query construction with method chaining.
//! - Support for:
//!   - `SELECT`, `JOIN`, `WHERE`, `GROUP BY`, `HAVING`, etc.
//!   - Conditional queries with `AND`, `OR`, `NOT`, `LIKE`, `IN`, `NOT IN`, `IS NULL`, etc.
//!   - Aggregation functions: `AVG`, `SUM`, `MIN`, `MAX`, and `COUNT`.
//!   - Date functions: `DATE`, `TIME`, `YEAR`, `MONTH`, `DAY`, etc.
//!   - Function aliases and raw expressions.
//!   - CRUD operations: `INSERT`, `UPDATE`, and `DELETE`.
//!   - Subqueries and nested conditions using closures.
//!   - Cursor-based pagination support via `paginate()`.
//!   - SQL query generation as raw `sql()` or formatted output `pretty_sql()`.
//!   - Query validation and error handling (can be skipped with `skip_validation()`).
//!
//! Use your IDE to explore the available methods, or refer to the [docs.rs/eloquent - QueryBuilder](https://docs.rs/eloquent/latest/eloquent/struct.QueryBuilder.html).
//!
//! ## Installation
//!
//! To use Eloquent, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! eloquent = "2.0"
//! ```
//!
//! ## Usage
//!
//! ### Simple query example
//!
//! This example demonstrates a basic SQL query using Eloquent's fluent API.
//!
//! ```rust
//! use eloquent::Eloquent;
//!
//! let query = Eloquent::query()
//!     .table("users")
//!     .select(vec!["name", "email"])
//!     .where_not_null("verified_at")
//!     .where_like("email", "%@gmail.com")
//!     .limit(100);
//!
//! println!("{}", query.pretty_sql()?);
//! ```
//!
//! ```sql
//! SELECT
//!     name,
//!     email
//! FROM
//!     users
//! WHERE
//!     verified_at IS NOT NULL
//!     AND email LIKE '%@gmail.com'
//! LIMIT
//!     100
//! ```
//!
//! ### Complex query example
//!
//! This example demonstrates a more complex SQL query using Eloquent's fluent API.
//!
//! ```rust
//! use eloquent::Eloquent;
//!
//! let query = Eloquent::query()
//!     .table("flights")
//!     .select("origin_airport")
//!     .select_avg("startup_time_in_minutes", "startup_time_in_minutes_avg")
//!     .select_as("airports.city", "destination_city")
//!     .join(
//!         "airports",
//!         "flights.destination_airport",
//!         "airports.iata_code",
//!     )
//!     .r#where("origin_airport", "AMS")
//!     .where_not_in("flight_number", vec!["KL123", "KL456"])
//!     .where_not_null("gate_number")
//!     .where_closure(|q| {
//!         q.where_gte("flight_duration", 120)
//!             .or_where_like("airports.city", "%NY%")
//!     })
//!     .group_by(vec!["origin_airport", "airports.city"])
//!     .having_gt("startup_time_in_minutes_avg", 120)
//!     .order_by_asc("startup_time_in_minutes_avg")
//!     .limit(20);
//!
//! println!("{}", query.pretty_sql()?);
//! ```
//!
//! ```sql
//! SELECT
//!     origin_airport,
//!     AVG(startup_time_in_minutes) AS startup_time_in_minutes_avg,
//!     airports.city AS destination_city
//! FROM
//!     flights
//!     JOIN airports ON flights.destination_airport = airports.iata_code
//! WHERE
//!     origin_airport = 'AMS'
//!     AND flight_number NOT IN ('KL123', 'KL456')
//!     AND gate_number IS NOT NULL
//!     AND (
//!         flight_duration >= 120
//!         OR airports.city LIKE '%NY%'
//!     )
//! GROUP BY
//!     origin_airport,
//!     airports.city
//! HAVING
//!     startup_time_in_minutes_avg > 120
//! ORDER BY
//!     startup_time_in_minutes_avg ASC
//! LIMIT
//!     20
//! ```
//!
//! ### Subquery example
//!
//! This example demonstrates a subquery using Eloquent's fluent API.
//!
//! ```rust
//! use eloquent::Eloquent;
//!
//! let subquery = Eloquent::subquery()
//!     .table("tickets")
//!     .select("event_id")
//!     .select_avg("price", "price_avg")
//!     .group_by("event_id")
//!     .order_by_desc("price_avg")
//!     .limit(1);
//!
//! let query = Eloquent::query()
//!     .table("events")
//!     .select(vec!["event_name", "event_date"])
//!     .r#where("event_id", subquery)
//!     .pretty_sql()?;
//! ```
//!
//! ```sql
//! SELECT
//!     event_name,
//!     event_date
//! FROM
//!     EVENTS
//! WHERE
//!     event_id = (
//!         SELECT
//!             event_id,
//!             AVG(price) AS price_avg
//!         FROM
//!             tickets
//!         GROUP BY
//!             event_id
//!         ORDER BY
//!             price_avg DESC
//!         LIMIT
//!             1
//!     )
//! ```
//!
//! ### Pagination example
//!
//! This example demonstrates cursor-based pagination using Eloquent's `paginate()` method.
//!
//! ```rust
//! use eloquent::Eloquent;
//!
//! let last_id = None; // initial query
//!
//! let query = Eloquent::query()
//!     .table("departures")
//!     .select("flight_number")
//!     .paginate::<u64>("id", last_id, 25)
//!     .sql()?;
//! ```
//!
//! ```sql
//! SELECT flight_number FROM departures LIMIT 25
//! ```
//!
//! ```rust
//! use eloquent::Eloquent;
//!
//! let last_id = Some(40); // last id from previous query
//!
//! let query = Eloquent::query()
//!     .table("departures")
//!     .select("flight_number")
//!     .paginate("id", last_id, 25)
//!     .sql()?;
//! ```
//!
//! ```sql
//! SELECT flight_number FROM departures WHERE id > 40 LIMIT 25
//! ```

pub use eloquent_core::*;

/// The main struct for building queries.
pub struct Eloquent;

impl Eloquent {
    /// Create a new query builder.
    pub fn query() -> QueryBuilder {
        QueryBuilder::new()
    }

    /// Create a new subquery builder.
    pub fn subquery() -> SubqueryBuilder {
        SubqueryBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_query() {
        let query = Eloquent::query().table("flights").select(vec![
            "origin_airport",
            "destination_airport",
            "flight_duration",
        ]);

        assert_eq!(
            query.sql().unwrap(),
            "SELECT origin_airport, destination_airport, flight_duration FROM flights"
        );
    }

    #[test]
    fn test_subquery() {
        let subquery = Eloquent::subquery()
            .table("flights")
            .select_avg("duration_in_min", "avg_duration_in_min");

        let query = Eloquent::query()
            .table("flights")
            .select_as(subquery, "avg_duration");

        assert_eq!(
            query.sql().unwrap(),
            "SELECT (SELECT AVG(duration_in_min) AS avg_duration_in_min FROM flights) AS avg_duration FROM flights"
        );
    }
}
