use crate::db::cached::CachedStmt;

pub trait PrepFields {
    fn fields(&self) -> Vec<std::string::String>;
}

pub trait PrepValues {
    fn values(&self) -> Vec<rusqlite::types::Value>;
}

pub trait TableName {
    fn table_name(&self) -> String;
}

pub trait PrepCreateUpdate: PrepFields + TableName + PrepValues {
    fn prepare(&self) -> CachedStmt {
        let fields = self.fields();
        let table_name = self.table_name();
        let placeholders = fields
            .iter()
            .map(|f| format!(":{}", f))
            .collect::<Vec<_>>()
            .join(", ");
        let columns = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
            table_name, columns, placeholders
        );
        CachedStmt {
            table: self.table_name(),
            query: query,
            params: self.values(),
        }
    }
}

pub trait PrepCreate: PrepFields + TableName + PrepValues {
    fn prepare(&self) -> CachedStmt {
        let fields = self.fields();
        let table_name = self.table_name();
        let placeholders = fields
            .iter()
            .map(|f| format!(":{}", f))
            .collect::<Vec<_>>()
            .join(", ");
        let columns = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns, placeholders
        );
        CachedStmt {
            table: self.table_name(),
            query: query,
            params: self.values(),
        }
    }
}

pub trait PrepUpdate: PrepFields + TableName + PrepValues {
    fn prepare(&self) -> CachedStmt {
        let mut fields = self.fields();
        let table_name = self.table_name();
        let id = fields.remove(0);
        let placeholders = fields
            .iter()
            .map(|f| format!(":{}", f))
            .collect::<Vec<_>>()
            .join(", ");
        let columns = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let replacements = columns
            .split(", ")
            .zip(placeholders.split(", "))
            .map(|(col, plc)| format!("{} = {}", col, plc))
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "UPDATE {} SET {} WHERE id = {}",
            table_name, replacements, id
        );
        CachedStmt {
            table: self.table_name(),
            query: query,
            params: self.values(),
        }
    }
}

pub trait PrepDelete: TableName {
    fn prepare(&self) -> CachedStmt {
        CachedStmt {
            table: self.table_name(),
            query: format!("DELETE FROM {} WHERE id = ?", self.table_name()),
            params: [].to_vec(),
        }
    }
}

pub trait ListTable: TableName {
    fn list_short(&self) -> CachedStmt {
        CachedStmt {
            table: self.table_name(),
            query: format!("SELECT id, name FROM {} ORDER BY id", self.table_name()),
            params: [].to_vec(),
        }
    }
}

pub trait ListID: TableName {
    fn list_long(&self, id: &i64) -> CachedStmt {
        CachedStmt {
            table: self.table_name(),
            query: format!(
                "SELECT * FROM {} WHERE id = {} ORDER BY id",
                self.table_name(),
                id
            ),
            params: [].to_vec(),
        }
    }
}
