use crate::{
    shared::{Clause, WhereOperator},
    Eloquent, Operator, Variable,
};

impl Eloquent {
    pub fn compile(&self) -> String {
        if self.bindings.insert.is_empty() == false {
            return self.compile_insert();
        }

        if self.bindings.update.is_empty() == false {
            return self.compile_update();
        }

        if self.bindings.is_delete {
            return self.compile_delete();
        }

        return self.compile_select();
    }

    fn compile_select(&self) -> String {
        let mut builder = "SELECT ".to_string();

        if self.bindings.select.is_empty() {
            builder.push_str("*");
        } else {
            builder.push_str(&self.bindings.select.join(", "));
        }

        builder.push_str(" FROM ");
        builder.push_str(&self.bindings.table);

        for join in self.bindings.join.iter() {
            builder.push_str(&format!(
                " {} {} ON {} = {}",
                join.r#type, join.table, join.left_hand, join.right_hand
            ));
        }

        builder = self.append_where_clauses(&mut builder);

        if self.bindings.group_by.is_empty() == false {
            builder.push_str(" GROUP BY ");
            builder.push_str(&self.bindings.group_by.join(", "));
        }

        if self.bindings.having.is_empty() == false {
            builder.push_str(" HAVING ");
            builder.push_str(
                &self
                    .bindings
                    .having
                    .clone()
                    .into_iter()
                    .map(|clause| format!("{} {} {}", clause.column, clause.operator, clause.value))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }

        if self.bindings.order_by.is_empty() == false {
            builder.push_str(" ORDER BY ");
            builder.push_str(&self.bindings.order_by.join(", "));
        }

        if let Some(limit) = self.bindings.limit {
            builder.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.bindings.offset {
            builder.push_str(&format!(" OFFSET {}", offset));
        }

        builder
    }

    fn compile_insert(&self) -> String {
        let mut builder = "INSERT INTO ".to_string();

        builder.push_str(&self.bindings.table);

        builder.push_str(" (");
        builder.push_str(
            &self
                .bindings
                .insert
                .clone()
                .into_iter()
                .map(|(column, _value)| column)
                .collect::<Vec<String>>()
                .join(", "),
        );
        builder.push_str(") VALUES (");
        builder.push_str(
            &self
                .bindings
                .insert
                .clone()
                .into_iter()
                .map(|(_column, value)| value.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        builder.push_str(")");

        if self.bindings.r#where.is_empty() == false {
            panic!("Where clauses are not allowed in insert statements.")
        }

        builder
    }

    fn compile_update(&self) -> String {
        let mut builder = "UPDATE ".to_string();

        builder.push_str(&self.bindings.table);

        builder.push_str(" SET ");
        builder.push_str(
            &self
                .bindings
                .update
                .clone()
                .into_iter()
                .map(|(column, value)| format!("{} = {}", column, value))
                .collect::<Vec<String>>()
                .join(", "),
        );

        builder = self.append_where_clauses(&mut builder);

        builder
    }

    fn compile_delete(&self) -> String {
        let mut builder = "DELETE FROM ".to_string();

        builder.push_str(&self.bindings.table);

        builder = self.append_where_clauses(&mut builder);

        builder
    }

    fn append_where_clauses(&self, builder: &mut String) -> String {
        for (index, clause) in self.bindings.r#where.iter().enumerate() {
            if index == 0 {
                builder.push_str(" WHERE ");
            } else if clause.where_operator == WhereOperator::And {
                builder.push_str(" AND ");
            } else {
                builder.push_str(" OR ");
            }

            if clause.where_operator == WhereOperator::Not {
                builder.push_str("NOT ");
            }

            builder.push_str(self.construct_where_clause(clause.clone().into()).as_str());
        }

        for (index, closure) in self.bindings.where_closure.iter().enumerate() {
            if index == 0 && self.bindings.r#where.is_empty() {
                builder.push_str(" WHERE ");
            } else if closure.where_operator == WhereOperator::And {
                builder.push_str(" AND ");
            } else {
                builder.push_str(" OR ");
            }

            builder.push_str("(");
            for (index, clause) in closure.closures.iter().enumerate() {
                if index == 0 {
                    //
                } else if clause.where_operator == WhereOperator::And {
                    builder.push_str(" AND ");
                } else {
                    builder.push_str(" OR ");
                }

                builder.push_str(self.construct_where_clause(clause.clone().into()).as_str());
            }
            builder.push_str(")");
        }

        builder.to_string()
    }

    fn construct_where_clause(&self, clauses: Clause) -> String {
        match clauses.value {
            Variable::Null => match clauses.operator {
                Operator::Equal => format!("{} IS NULL", clauses.column),
                Operator::NotEqual => format!("{} IS NOT NULL", clauses.column),
                _ => panic!("Invalid operator for NULL value in WHERE clause."),
            },
            _ => format!("{} {} {}", clauses.column, clauses.operator, clauses.value),
        }
    }
}
