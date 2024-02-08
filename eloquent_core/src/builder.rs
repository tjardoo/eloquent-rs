use crate::{Clause, Direction, Eloquent, Operator, Variable};

pub struct Bindings {
    pub select: Vec<String>,
    pub insert: Vec<(String, Variable)>,
    pub update: Vec<(String, Variable)>,
    pub table: Option<String>,
    // pub join: Vec<String>,
    pub r#where: Vec<Clause>,
    // pub group_by: Vec<String>,
    // pub having: Vec<String>,
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

    pub fn r#where(&mut self, column: String, operator: Operator, value: Variable) -> &mut Self {
        self.bindings.r#where.push(Clause {
            column,
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
