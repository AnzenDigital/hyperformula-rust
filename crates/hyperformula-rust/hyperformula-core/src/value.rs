use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    String(String),
    // Keep both variant names to remain compatible with older code/tests.
    Bool(bool),
    Boolean(bool),
    Error(String),
    Empty,
    // Older code expected a Range (flat Vec<Value>);
    // other parts of the codebase use Array (2D Vec).
    Range(Vec<Value>),
    Array(Vec<Vec<Value>>),
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Error(e) => e.clone(),
            Value::Empty => String::new(),
            Value::Range(r) => format!(
                "[{}]",
                r.iter()
                    .map(|v| v.as_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Array(_) => "#VALUE!".to_string(),
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Boolean(b) => Some(*b),
            Value::Number(n) => Some(*n != 0.0),
            Value::String(s) if s.eq_ignore_ascii_case("true") => Some(true),
            Value::String(s) if s.eq_ignore_ascii_case("false") => Some(false),
            _ => None,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Value::Error(_))
    }

    /// Flatten Range or Array into a flat iterator of Values.
    /// Range is a flat Vec<Value>, Array is Vec<Vec<Value>> representing rows.
    pub fn flatten(&self) -> Vec<Value> {
        match self {
            Value::Range(v) => v.iter().flat_map(|item| item.flatten()).collect(),
            Value::Array(rows) => rows
                .iter()
                .flat_map(|r| r.iter().flat_map(|item| item.flatten()))
                .collect(),
            _ => vec![self.clone()],
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        // Prefer Bool variant for compatibility with tests and generated code that uses Value::Bool.
        Value::Bool(b)
    }
}
