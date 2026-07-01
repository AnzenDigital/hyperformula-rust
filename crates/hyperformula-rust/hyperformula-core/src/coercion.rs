use crate::{Error, Value};

pub trait Coercion {
    fn to_number(&self) -> Result<f64, Error>;
    fn to_bool(&self) -> Result<bool, Error>;
    fn to_spreadsheet_string(&self) -> String;
}

impl Coercion for Value {
    fn to_number(&self) -> Result<f64, Error> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Bool(true) => Ok(1.0),
            Value::Bool(false) => Ok(0.0),
            Value::String(s) => s
                .trim()
                .parse::<f64>()
                .map_err(|_| Error::Value(format!("Cannot convert '{}' to number", s))),
            Value::Empty => Ok(0.0),
            Value::Error(e) => Err(Error::Value(format!("Error value: {:?}", e))),
            Value::Range(_) => Err(Error::Value("Cannot convert range to number".to_string())),
        }
    }

    fn to_bool(&self) -> Result<bool, Error> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Number(n) => Ok(*n != 0.0),
            Value::String(s) => {
                let s_lower = s.to_lowercase();
                match s_lower.as_str() {
                    "true" => Ok(true),
                    "false" => Ok(false),
                    _ => Err(Error::Value(format!("Cannot convert '{}' to boolean", s))),
                }
            }
            Value::Empty => Ok(false),
            Value::Error(e) => Err(Error::Value(format!("Error value: {:?}", e))),
            Value::Range(_) => Err(Error::Value("Cannot convert range to boolean".to_string())),
        }
    }

    fn to_spreadsheet_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string().to_uppercase(),
            Value::String(s) => s.clone(),
            Value::Empty => String::new(),
            Value::Error(e) => format!("#ERROR:{:?}", e),
            Value::Range(r) => {
                format!(
                    "[{}]",
                    r.iter()
                        .map(Coercion::to_spreadsheet_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number() {
        assert_eq!(Value::Number(42.0).to_number().unwrap(), 42.0);
        assert_eq!(Value::Bool(true).to_number().unwrap(), 1.0);
        assert_eq!(Value::Bool(false).to_number().unwrap(), 0.0);
        assert_eq!(Value::String("123".to_string()).to_number().unwrap(), 123.0);
        assert_eq!(Value::Empty.to_number().unwrap(), 0.0);
        assert!(Value::String("abc".to_string()).to_number().is_err());
    }

    #[test]
    fn test_to_bool() {
        assert!(Value::Bool(true).to_bool().unwrap());
        assert!(Value::Number(1.0).to_bool().unwrap());
        assert!(!Value::Number(0.0).to_bool().unwrap());
        assert!(Value::String("true".to_string()).to_bool().unwrap());
        assert!(!Value::String("FALSE".to_string()).to_bool().unwrap());
        assert!(!Value::Empty.to_bool().unwrap());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Value::Number(42.5).to_spreadsheet_string(), "42.5");
        assert_eq!(Value::Bool(true).to_spreadsheet_string(), "TRUE");
        assert_eq!(
            Value::String("hello".to_string()).to_spreadsheet_string(),
            "hello"
        );
        assert_eq!(Value::Empty.to_spreadsheet_string(), "");
    }
}
