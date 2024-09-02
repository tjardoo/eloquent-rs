use crate::{Join, JoinType, SubqueryBuilder};

impl SubqueryBuilder {
    fn add_join(
        mut self,
        table: &str,
        left_hand: &str,
        right_hand: &str,
        join_type: JoinType,
    ) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            left_hand: left_hand.to_string(),
            join_type,
            right_hand: right_hand.to_string(),
        });

        self
    }

    pub fn join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Inner)
    }

    pub fn left_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Left)
    }

    pub fn right_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Right)
    }

    pub fn full_join(self, table: &str, left_hand: &str, right_hand: &str) -> Self {
        self.add_join(table, left_hand, right_hand, JoinType::Full)
    }
}
