use crate::Eloquent;

impl Eloquent {
    pub fn compile(&self) -> String {
        if self.bindings.table.is_none() {
            panic!("No table specified for the query.");
        }

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
        builder.push_str(&self.bindings.table.as_ref().unwrap());

        builder = self.append_where_clauses(&mut builder);

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

        builder.push_str(&self.bindings.table.as_ref().unwrap());

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

        builder.push_str(&self.bindings.table.as_ref().unwrap());

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

        builder.push_str(&self.bindings.table.as_ref().unwrap());

        builder = self.append_where_clauses(&mut builder);

        builder
    }

    fn append_where_clauses(&self, builder: &mut String) -> String {
        for (index, clause) in self.bindings.r#where.iter().enumerate() {
            if index == 0 {
                builder.push_str(" WHERE ");
            } else {
                builder.push_str(" AND ");
            }

            builder.push_str(&format!(
                "{} {} {}",
                clause.column, clause.operator, clause.value
            ));
        }

        builder.to_string()
    }
}
