use crate::{
    Clause, Direction, Eloquent, Join, JoinType, Operator, Variable, WhereClause, WhereClosure,
    WhereOperator,
};

pub struct Bindings {
    pub select: Vec<String>,
    pub insert: Vec<(String, Variable)>,
    pub update: Vec<(String, Variable)>,
    pub table: Option<String>,
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
    pub fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        for column in columns.iter() {
            self.bindings.select.push(column.to_string());
        }

        self
    }

    pub fn select_count(&mut self, column: &str, alias: &str) -> &mut Self {
        self.bindings
            .select
            .push(format!("COUNT({}) AS {}", column, alias.to_string()));

        self
    }

    pub fn select_max(&mut self, column: &str, alias: &str) -> &mut Self {
        self.bindings
            .select
            .push(format!("MAX({}) AS {}", column, alias.to_string()));

        self
    }

    pub fn select_min(&mut self, column: &str, alias: &str) -> &mut Self {
        self.bindings
            .select
            .push(format!("MIN({}) AS {}", column, alias.to_string()));

        self
    }

    pub fn select_sum(&mut self, column: &str, alias: &str) -> &mut Self {
        self.bindings
            .select
            .push(format!("SUM({}) AS {}", column, alias.to_string()));

        self
    }

    pub fn select_avg(&mut self, column: &str, alias: &str) -> &mut Self {
        self.bindings
            .select
            .push(format!("AVG({}) AS {}", column, alias.to_string()));

        self
    }

    pub fn insert(&mut self, columns: Vec<(&str, Variable)>) -> &mut Self {
        for column in columns.iter() {
            self.bindings
                .insert
                .push((column.0.to_string(), column.1.clone()));
        }

        self
    }

    pub fn update(&mut self, columns: Vec<(&str, Variable)>) -> &mut Self {
        for column in columns.iter() {
            self.bindings
                .update
                .push((column.0.to_string(), column.1.clone()));
        }

        self
    }

    pub fn delete(&mut self) -> &mut Self {
        self.bindings.is_delete = true;

        self
    }

    pub fn table(&mut self, table: &str) -> &mut Self {
        self.bindings.table = Some(table.to_string());

        self
    }

    pub fn join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Inner);

        self
    }

    pub fn left_join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Left);

        self
    }

    pub fn right_join(&mut self, table: &str, left_hand: &str, right_hand: &str) -> &mut Self {
        self.create_join(table, left_hand, right_hand, JoinType::Right);

        self
    }

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

    pub fn r#where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.r#where.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator: WhereOperator::And,
        });

        self
    }

    pub fn or_where(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.r#where.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator: WhereOperator::Or,
        });

        self
    }

    pub fn where_not(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.r#where.push(WhereClause {
            column: column.to_string(),
            operator,
            value,
            where_operator: WhereOperator::Not,
        });

        self
    }

    pub fn where_closure(&mut self, closure: Vec<Clause>) -> &mut Self {
        self.bindings.where_closure.push(WhereClosure {
            closure,
            where_operator: WhereOperator::And,
        });

        self
    }

    pub fn or_where_closure(&mut self, closure: Vec<Clause>) -> &mut Self {
        self.bindings.where_closure.push(WhereClosure {
            closure,
            where_operator: WhereOperator::Or,
        });

        self
    }

    pub fn group_by(&mut self, column: &str) -> &mut Self {
        self.bindings.group_by.push(column.to_string());

        self
    }

    pub fn having(&mut self, column: &str, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.having.push(Clause {
            column: column.to_string(),
            operator,
            value,
        });

        self
    }

    pub fn order_by(&mut self, column: &str, direction: Direction) -> &mut Self {
        self.bindings
            .order_by
            .push(format!("{} {}", column, direction));

        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.bindings.limit = Some(limit);

        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.bindings.offset = Some(offset);

        self
    }

    pub fn to_sql(&mut self) -> String {
        self.compile()
    }
}
