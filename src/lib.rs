//! # Eloquent
//!
//! Eloquent is a SQL query builder to easily build complex SQL queries in Rust. It is inspired by Laravel's Query Builder and is designed to be simple and easy to use.
//!
//! ```rust
//! use eloquent_core::{ArrayVariable, Direction, Eloquent, Operator, Variable};
//!
//! fn example_query() {
//!     let query = Eloquent::table("orders")
//!         .select("orders.customer_id")
//!         .select_as("customers.name", "customer_name")
//!         .select_count("orders.id", "total_orders")
//!         .join("customers", "orders.customer_id", "customers.id")
//!         .r#where(
//!             "orders.order_date",
//!             Operator::GreaterThanOrEqual,
//!             Variable::String("2024-01-01".to_string()),
//!         )
//!         .r#where(
//!             "customers.country_id",
//!             Operator::In,
//!             Variable::Array(vec![
//!                 ArrayVariable::String("NL".to_string()),
//!                 ArrayVariable::String("DE".to_string()),
//!             ]),
//!         )
//!         .where_not_null("shipped_at")
//!         .group_by(vec!["orders.customer_id", "customers.name"])
//!         .having("total_orders", Operator::GreaterThan, Variable::Int(5))
//!         .order_by("total_orders", Direction::Desc)
//!         .order_by("customer_name", Direction::Asc)
//!         .limit(10)
//!         .offset(0)
//!         .to_sql();
//!
//!     assert_eq!(
//!         query,
//!         "SELECT orders.customer_id, customers.name AS customer_name, COUNT(orders.id) AS total_orders FROM orders JOIN customers ON orders.customer_id = customers.id WHERE orders.order_date >= `2024-01-01` AND customers.country_id IN (`NL`, `DE`) AND shipped_at IS NOT NULL GROUP BY orders.customer_id, customers.name HAVING total_orders > 5 ORDER BY total_orders DESC, customer_name ASC LIMIT 10 OFFSET 0"
//!     );
//! }
//! ```
//!
//! ```rust
//! use eloquent_core::{Eloquent, Operator, Variable};
//!
//! fn example_query() {
//!     let query = Eloquent::table("users")
//!         .where_closure(|closure| {
//!             closure
//!                 .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
//!                 .r#where("age", Operator::LessThan, Variable::Int(25));
//!         })
//!         .or_where_closure(|closure| {
//!             closure
//!                 .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(30))
//!                 .r#where("age", Operator::LessThan, Variable::Int(35));
//!         })
//!         .to_sql();
//!
//!     assert_eq!(
//!         query,
//!         "SELECT * FROM users WHERE (age >= 18 AND age < 25) OR (age >= 30 AND age < 35)"
//!     );
//! }
//! ```
