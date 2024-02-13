use crate::{
    select_columns::SelectColumns,
    shared::{
        Clause, FunctionType, Join, JoinType, WhereClause, WhereClauseBuilder, WhereClosure,
        WhereOperator,
    },
    Direction, Eloquent, Operator, Variable,
};

pub struct Bindings {
    pub select: Vec<String>,
    pub insert: Vec<(String, Variable)>,
    pub update: Vec<(String, Variable)>,
    pub table: String,
    pub join: Vec<Join>,
    pub r#where: Vec<WhereClause>,
    pub where_closure: Vec<WhereClosure>,
    pub group_by: Vec<String>,
    pub having: Vec<Clause>,
    pub order_by: Vec<String>,
    pub is_delete: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Eloquent {
    pub fn select<T>(&mut self, columns: T) -> &mut Self
    where
        T: SelectColumns,
    {
        let columns = columns.to_columns();

        for column in columns.iter() {
            self.bindings.select.push(column.to_string());
        }

        self
    }

    /// Select a count of the given column and give it an alias.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select_count("id", "total_users");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT COUNT(id) AS total_users FROM users");
    /// ```
    pub fn select_count(&mut self, column: &str, alias: &str) -> &mut Self {
        self.create_function(column, alias, FunctionType::Count);

        self
    }

    /// Select the max of the given column and give it an alias.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select_max("id", "max_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT MAX(id) AS max_id FROM users");
    /// ```
    pub fn select_max(&mut self, column: &str, alias: &str) -> &mut Self {
        self.create_function(column, alias, FunctionType::Max);

        self
    }

    /// Select the min of the given column and give it an alias.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select_min("id", "min_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT MIN(id) AS min_id FROM users");
    /// ```
    pub fn select_min(&mut self, column: &str, alias: &str) -> &mut Self {
        self.create_function(column, alias, FunctionType::Min);

        self
    }

    /// Select the sum of the given column and give it an alias.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select_sum("id", "sum_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT SUM(id) AS sum_id FROM users");
    /// ```
    pub fn select_sum(&mut self, column: &str, alias: &str) -> &mut Self {
        self.create_function(column, alias, FunctionType::Sum);

        self
    }

    /// Select the avg of the given column and give it an alias.
    ///
    /// ```rust
    /// use eloquent_core::Eloquent;
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.select_avg("id", "avg_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT AVG(id) AS avg_id FROM users");
    /// ```
    pub fn select_avg(&mut self, column: &str, alias: &str) -> &mut Self {
        self.create_function(column, alias, FunctionType::Avg);

        self
    }

    fn create_function(&mut self, column: &str, alias: &str, function: FunctionType) -> &mut Self {
        self.bindings
            .select
            .push(format!("{}({}) AS {}", function, column, alias.to_string()));

        self
    }

    /// Insert a new column and value into the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.insert("name", Variable::String("John Doe".to_string()));
    ///
    /// assert_eq!(eloquent.to_sql(), "INSERT INTO users (name) VALUES (`John Doe`)");
    /// ```
    pub fn insert(&mut self, column: &str, value: Variable) -> &mut Self {
        self.bindings.insert.push((column.to_string(), value));

        self
    }

    /// Insert multiple columns and values into the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.insert_many(vec![
    ///     ("first_name", Variable::String("John".to_string())),
    ///     ("last_name", Variable::String("Doe".to_string())),
    /// ]);
    ///
    /// assert_eq!(eloquent.to_sql(), "INSERT INTO users (first_name, last_name) VALUES (`John`, `Doe`)");
    /// ```
    pub fn insert_many(&mut self, columns: Vec<(&str, Variable)>) -> &mut Self {
        for column in columns.iter() {
            self.bindings
                .insert
                .push((column.0.to_string(), column.1.clone()));
        }

        self
    }

    /// Update a column with a new value in the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.update("name", Variable::String("John Doe".to_string()));
    ///
    /// assert_eq!(eloquent.to_sql(), "UPDATE users SET name = `John Doe`");
    /// ```
    pub fn update(&mut self, column: &str, value: Variable) -> &mut Self {
        self.bindings
            .update
            .push((column.to_string(), value.clone()));

        self
    }

    /// Update multiple columns with new values in the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.update_many(vec![
    ///     ("first_name", Variable::String("John".to_string())),
    ///     ("last_name", Variable::String("Doe".to_string())),
    /// ]);
    ///
    /// assert_eq!(eloquent.to_sql(), "UPDATE users SET first_name = `John`, last_name = `Doe`");
    /// ```
    pub fn update_many(&mut self, columns: Vec<(&str, Variable)>) -> &mut Self {
        for (column, value) in columns.iter() {
            self.bindings
                .update
                .push((column.to_string(), value.clone()));
        }

        self
    }

    /// Deletes records from the table.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.delete();
    ///
    /// assert_eq!(eloquent.to_sql(), "DELETE FROM users");
    /// ```
    pub fn delete(&mut self) -> &mut Self {
        self.bindings.is_delete = true;

        self
    }

    /// Add a join clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.join("addresses", "users.id", "addresses.user_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users JOIN addresses ON users.id = addresses.user_id");
    /// ```
    pub fn join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Inner);

        self
    }

    /// Add a left join clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.left_join("addresses", "users.id", "addresses.user_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users LEFT JOIN addresses ON users.id = addresses.user_id");
    /// ```
    pub fn left_join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Left);

        self
    }

    /// Add a right join clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.right_join("addresses", "users.id", "addresses.user_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users RIGHT JOIN addresses ON users.id = addresses.user_id");
    /// ```
    pub fn right_join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Right);

        self
    }

    /// Add a full join clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.full_join("addresses", "users.id", "addresses.user_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users FULL JOIN addresses ON users.id = addresses.user_id");
    /// ```
    pub fn full_join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Full);

        self
    }

    pub fn create_join(
        &mut self,
        table: &str,
        left_hand: &str,
        right_hand: &str,
        r#type: JoinType,
    ) -> &mut Self {
        self.bindings.join.push(Join {
            table: table.to_string(),
            left_hand: left_hand.to_string(),
            right_hand: right_hand.to_string(),
            r#type,
        });

        self
    }

    fn create_where_clause(
        &mut self,
        column: &str,
        operator: Operator,
        value: Variable,
        where_operator: WhereOperator,
    ) -> &mut Self {
        self.bindings.r#where.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator,
        });

        self
    }

    pub fn where_closure<C>(&mut self, closure: C) -> &mut Self
    where
        C: FnOnce(&mut WhereClosure),
    {
        let mut builder = WhereClosure::new(WhereOperator::And);

        closure(&mut builder);

        self.bindings.where_closure.push(builder);

        self
    }

    pub fn or_where_closure<C>(&mut self, closure: C) -> &mut Self
    where
        C: FnOnce(&mut WhereClosure),
    {
        let mut builder = WhereClosure::new(WhereOperator::Or);

        closure(&mut builder);

        self.bindings.where_closure.push(builder);

        self
    }

    /// Add a "group by" clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.group_by("country_id");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users GROUP BY country_id");
    /// ```
    pub fn group_by(&mut self, column: &str) -> &mut Self {
        self.bindings.group_by.push(column.to_string());

        self
    }

    /// Add a "having" clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.having("created_at", Operator::GreaterThanOrEqual, Variable::String("2024-01-01".to_string()));
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users HAVING created_at >= `2024-01-01`");
    /// ```
    pub fn having(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.having.push(Clause {
            column: column.to_string(),
            operator,
            value,
        });

        self
    }

    /// Add an "order by" clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Direction};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.order_by("country_id", Direction::Asc);
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users ORDER BY country_id ASC");
    /// ```
    pub fn order_by(&mut self, column: &str, direction: Direction) -> &mut Self {
        self.bindings
            .order_by
            .push(format!("{} {}", column, direction));

        self
    }

    /// Add a "limit" clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.limit(100);
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users LIMIT 100");
    /// ```
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.bindings.limit = Some(limit);

        self
    }

    /// Add a "offset" clause to the query.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.offset(1000);
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users OFFSET 1000");
    /// ```
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.bindings.offset = Some(offset);

        self
    }

    /// Compile the query into a SQL string.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent};
    ///
    /// let mut eloquent = Eloquent::table("users");
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users");
    /// ```
    pub fn to_sql(&mut self) -> String {
        self.compile()
    }
}

impl WhereClauseBuilder for Eloquent {
    fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::And);

        self
    }

    fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Or);

        self
    }

    fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Not);

        self
    }

    fn where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::And);

        self
    }

    fn where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::And,
        );

        self
    }

    fn or_where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::Or);

        self
    }

    fn or_where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::Or,
        );

        self
    }
}
