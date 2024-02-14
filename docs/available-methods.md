# Available Methods

- [Table](#table)
- [Select](#select)
- [Insert](#insert)
- [Update](#update)
- [Delete](#delete)
- [Where](#where)
- [Where Closure](#where-closure)
- [Join](#join)
- [Group By](#group-by)
- [Having](#having)
- [Order By](#order-by)
- [Limit and Offset](#limit-and-offset)
- [To SQL](#to-sql)

## Table

- ``table("users")``

## Select

- ``select("id")``
- ``select(vec!["name", "email"])``
- ``select_as("address.country", "address_country")``
- ``select_count("id", "total_users")``
- ``select_max("id", "max_id")``
- ``select_min("id", "min_id")``
- ``select_sum("id", "sum_id")``
- ``select_avg("id", "avg_id")``

## Insert

- ``insert("name", Variable::String("John Doe".to_string()))``
- ``insert_many(vec![("first_name", Variable::String("John".to_string())), ("last_name", Variable::String("Doe".to_string()))])``

## Update

- ``update("name", Variable::String("John Doe".to_string()))``
- ``update_many(vec![("first_name", Variable::String("John".to_string())), ("last_name", Variable::String("Doe".to_string()))])``

## Delete

- ``.delete()``

## Where

- ``r#where("id", Operator::Equal, Variable::Int(100))``
- ``or_where("id", Operator::Equal, Variable::Int(200))``
- ``where_not("country_code", Operator::Equal, Variable::String("NL".to_string()))``
- ``where_null("country_id")``
- ``where_not_null("country_id")``
- ``or_where_null("country_code")``
- ``or_where_not_null("verified_at")``

## Where Closure

- ``where_closure(|closure| { closure.r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18)).r#where("age", Operator::LessThan, Variable::Int(25)); });``
- ``or_where_closure(|closure| { closure.r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18)).r#where("age", Operator::LessThan, Variable::Int(25)); })``

## Join

- ``join("addresses", "users.id", "addresses.user_id")``
- ``left_join("addresses", "users.id", "addresses.user_id")``
- ``right_join("addresses", "users.id", "addresses.user_id")``
- ``full_join("addresses", "users.id", "addresses.user_id")``

## Group By

- ``group_by("country_id")``
- ``group_by(vec!["country_id", "city_id"])``

## Having

- ``having("total_orders", Operator::GreaterThanOrEqual, Variable::Int(5))``

## Order By

- ``order_by("country_id", Direction::Asc)``

## Limit and Offset

- ``limit(100)``
- ``offset(1000)``

## To SQL

- ``to_sql()``
