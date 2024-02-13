# Available Methods

- [Table](#table)
- [Select](#select)
- [Insert](#insert)
- [Update](#update)
- [Delete](#delete)
- [Where](#where)
- [Join](#join)
- [Group By](#group-by)
- [Having](#having)
- [Order By](#order-by)
- [Limit & Offset](#limit-offset)
- [To SQL](#to-sql)

## Table

- ``table("users")``

## Select

- ``select("id")``
- ``select(vec!["id", "name"])``
- ``select_count("id", "total_users")``
- ``select_max("id", "max_id")``
- ``select_min("id", "min_id")``
- ``select_avg("id", "avg_id")``
- ``select_sum("id", "sum_id")``

## Insert

- ``insert("name", Variable::new("John Doe"))``
- ``insert_many(vec![("first_name", Variable::new("John")), ("last_name", Variable::new("Doe"))])``

## Update

- ``update("name", Variable::new("John Doe"))``
- ``update_many(vec![("first_name", Variable::new("John")), ("last_name", Variable::new("Doe"))])``

## Delete

- ``delete()``

## Where

- ``r#where()``
- ``or_where()``
- ``where_not()``
- ``where_null()``
- ``where_not_null()``
- ``or_where_null()``
- ``or_where_not_null()``
- ``where_closure(|closure|)``
- ``or_where_closure(|closure|)``

## Join

- ``join("addresses", "users.id", "addresses.user_id")``
- ``left_join("addresses", "users.id", "addresses.user_id")``
- ``right_join("addresses", "users.id", "addresses.user_id")``
- ``full_join("addresses", "users.id", "addresses.user_id")``

## Group By

- ``group_by("country_id")``

## Having

- ``having("created_at", Operator::GreaterThanOrEqual, Variable::String("2024-01-01".to_string()))``

## Order By

- ``order_by("id", Direction::Desc)``

## Limit & Offset {#limit-offset}

- ``limit(100)``
- ``offset(100)``

## To SQL

``to_sql()``
