use crate::{
    shared::{Closures, WhereClause, WhereOperator},
    Operator, Variable,
};

impl Closures {
    pub fn new(where_operator: WhereOperator) -> Self {
        Self {
            closures: Vec::new(),
            where_operator,
        }
    }

    /// Add a "where" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
    ///    .r#where("age", Operator::LessThan, Variable::Int(25));
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (age >= 18 AND age < 25)");
    /// ```
    pub fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::And);

        self
    }

    /// Add a "or where" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .r#where("age", Operator::GreaterThanOrEqual, Variable::Int(18))
    ///    .or_where("is_verified", Operator::Equal, Variable::Bool(true));
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (age >= 18 OR is_verified = true)");
    /// ```
    pub fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Or);

        self
    }

    /// Add a "where not" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .where_not("is_verified", Operator::Equal, Variable::Bool(false));
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE NOT (is_verified = false)");
    /// ```
    pub fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.create_where_clause(column, operator, value, WhereOperator::Not);

        self
    }

    /// Add a "where null" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .where_null("verified_at");
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (verified_at IS NULL)");
    /// ```
    pub fn where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::And);

        self
    }

    /// Add a "where not null" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .where_not_null("verified_at");
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (verified_at IS NOT NULL)");
    /// ```
    pub fn where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::And,
        );

        self
    }

    /// Add a "or where null" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .r#where("country_code", Operator::Equal, Variable::String("NL".to_string()))
    ///    .or_where_null("country_code");
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (country_code = `NL` OR country_code IS NULL)");
    /// ```
    pub fn or_where_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(column, Operator::Equal, Variable::Null, WhereOperator::Or);

        self
    }

    /// Add a "or where not null" clause to the closure.
    ///
    /// ```rust
    /// use eloquent_core::{Eloquent, Operator, Variable};
    ///
    /// let mut eloquent = Eloquent::table("users");
    /// eloquent.where_closure(|closure| {
    ///    closure
    ///    .r#where("country_code", Operator::Equal, Variable::String("NL".to_string()))
    ///    .or_where_not_null("verified_at");
    /// });
    ///
    /// assert_eq!(eloquent.to_sql(), "SELECT * FROM users WHERE (country_code = `NL` OR verified_at IS NOT NULL)");
    /// ```
    pub fn or_where_not_null(&mut self, column: &str) -> &mut Self {
        self.create_where_clause(
            column,
            Operator::NotEqual,
            Variable::Null,
            WhereOperator::Or,
        );

        self
    }

    fn create_where_clause(
        &mut self,
        column: &str,
        operator: Operator,
        value: Variable,
        where_operator: WhereOperator,
    ) -> &mut Self {
        self.closures.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator,
        });

        self
    }
}
