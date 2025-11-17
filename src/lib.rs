use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Write, Read};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalDBError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("SQL error: {0}")]
    SqlError(String),
}

pub type Result<T> = std::result::Result<T, LocalDBError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalDBValue {
    INT(i64),
    TEXT(String),
    UUID(String),
}

#[derive(Debug)]
pub struct LocalDB {
    pub path: String,
}

impl LocalDB {
    /// Create a new DB file
    pub fn create(path: &str) -> Result<Self> {
        fs::write(path, "{}")
            .map_err(|e| LocalDBError::IoError(e.to_string()))?;

        Ok(Self {
            path: path.to_string(),
        })
    }

    /// Open existing DB
    pub fn open(path: &str) -> Result<Self> {
        if !std::path::Path::new(path).exists() {
            return Err(LocalDBError::IoError("Database file not found".into()));
        }

        Ok(Self {
            path: path.to_string(),
        })
    }

    /// Add SQL lines into a single block
    pub fn add_lines(&self, lines: [&str; 2]) -> String {
        let mut out = String::new();
        for l in lines {
            out.push_str(l);
            out.push('\n');
        }
        out
    }

    /// Execute SQL statements
    pub fn exec(&mut self, sql: String) -> Result<()> {
        let statements = sql.split(";");

        for raw in statements {
            let stmt = raw.trim();
            if stmt.is_empty() {
                continue;
            }

            if stmt.starts_with("CREATE TABLE") {
                self.handle_create_table(stmt)?;
            } else if stmt.starts_with("INSERT INTO") || stmt.starts_with("INSET INTO") {
                self.handle_insert(stmt)?;
            } else {
                return Err(LocalDBError::SqlError(format!("Unsupported SQL: {}", stmt)));
            }
        }

        Ok(())
    }

    /// SELECT * FROM table;
    pub fn query(&self, sql: &str) -> Result<Vec<HashMap<String, LocalDBValue>>> {
        let sql = sql.trim();

        if !sql.starts_with("SELECT") {
            return Err(LocalDBError::SqlError("Only SELECT is supported".into()));
        }

        let table = Self::extract_table_name_from_select(sql)?;

        let content = fs::read_to_string(&self.path)
            .map_err(|e| LocalDBError::IoError(e.to_string()))?;

        let data: HashMap<String, Vec<HashMap<String, LocalDBValue>>> =
            serde_json::from_str(&content).unwrap_or_default();

        Ok(data.get(&table).cloned().unwrap_or_default())
    }

    // ========================= INTERNAL HANDLERS =============================

    fn handle_create_table(&self, sql: &str) -> Result<()> {
        let name = Self::extract_table_name_from_create(sql)?;

        let mut data: HashMap<String, Vec<HashMap<String, LocalDBValue>>> =
            Self::load_json(&self.path)?;

        if !data.contains_key(&name) {
            data.insert(name, vec![]);
        }

        Self::save_json(&self.path, &data)?;

        Ok(())
    }

    fn handle_insert(&self, sql: &str) -> Result<()> {
        let sql_fixed = sql.replace("INSET", "INSERT");

        let (table, (uuid, name)) = Self::extract_insert_data(&sql_fixed)?;

        let mut data: HashMap<String, Vec<HashMap<String, LocalDBValue>>> =
            Self::load_json(&self.path)?;

        if !data.contains_key(&table) {
            data.insert(table.clone(), vec![]);
        }

        let mut row = HashMap::new();
        row.insert("id".into(), LocalDBValue::UUID(uuid));
        row.insert("name".into(), LocalDBValue::TEXT(name));

        data.get_mut(&table).unwrap().push(row);

        Self::save_json(&self.path, &data)?;

        Ok(())
    }

    // ========================= JSON HELPERS =============================

    fn load_json(path: &str) -> Result<HashMap<String, Vec<HashMap<String, LocalDBValue>>>> {
        let content = fs::read_to_string(path)
            .map_err(|e| LocalDBError::IoError(e.to_string()))?;

        let json = serde_json::from_str(&content).unwrap_or_default();
        Ok(json)
    }

    fn save_json(
        path: &str,
        data: &HashMap<String, Vec<HashMap<String, LocalDBValue>>>
    ) -> Result<()> {
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| LocalDBError::IoError(e.to_string()))?;

        fs::write(path, json)
            .map_err(|e| LocalDBError::IoError(e.to_string()))?;

        Ok(())
    }

    // ========================= PARSE HELPERS =============================

    fn extract_table_name_from_create(sql: &str) -> Result<String> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        Ok(parts[2].to_string())
    }

    fn extract_table_name_from_select(sql: &str) -> Result<String> {
        let parts: Vec<&str> = sql.split_whitespace().collect();

        if parts.len() < 4 {
            return Err(LocalDBError::SqlError("Invalid SELECT syntax".into()));
        }

        Ok(parts[3].replace(";", ""))
    }

    fn extract_insert_data(sql: &str) -> Result<(String, (String, String))> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        let table = parts[2].to_string();

        let start = sql.find("(").unwrap() + 1;
        let end = sql.find(")").unwrap();
        let params = &sql[start..end];

        let list: Vec<&str> = params.split(",").collect();

        let uuid_raw = list[0].trim().replace("'", "");
        let name_raw = list[1].trim().replace("'", "");

        Ok((table, (uuid_raw, name_raw)))
    }
}
