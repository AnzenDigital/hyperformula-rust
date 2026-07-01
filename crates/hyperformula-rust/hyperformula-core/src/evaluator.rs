use crate::{Error, Value};
use std::collections::HashMap;

pub type FunctionImpl = Box<dyn Fn(&[Value]) -> Result<Value, Error> + Send + Sync>;

pub struct Evaluator {
    functions: HashMap<String, FunctionImpl>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register_function<F>(&mut self, name: &str, func: F)
    where
        F: Fn(&[Value]) -> Result<Value, Error> + Send + Sync + 'static,
    {
        self.functions.insert(name.to_uppercase(), Box::new(func));
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_uppercase())
    }

    pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value, Error> {
        let name_upper = name.to_uppercase();
        match self.functions.get(&name_upper) {
            Some(func) => func(args),
            None => Err(Error::NameError(format!("Function '{}' not found", name))),
        }
    }

    pub fn list_functions(&self) -> Vec<String> {
        let mut names: Vec<_> = self.functions.keys().cloned().collect();
        names.sort();
        names
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_call_function() {
        let mut evaluator = Evaluator::new();

        evaluator.register_function("DOUBLE", |args| {
            if args.len() != 1 {
                return Err(Error::ValueError("DOUBLE expects 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n * 2.0)),
                _ => Err(Error::ValueError("DOUBLE expects a number".to_string())),
            }
        });

        let result = evaluator.call_function("DOUBLE", &[Value::Number(21.0)]);
        assert_eq!(result.unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_function_not_found() {
        let evaluator = Evaluator::new();
        let result = evaluator.call_function("UNKNOWN", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_case_insensitive() {
        let mut evaluator = Evaluator::new();
        evaluator.register_function("TEST", |_| Ok(Value::Number(1.0)));

        assert!(evaluator.has_function("test"));
        assert!(evaluator.has_function("TEST"));
        assert!(evaluator.has_function("TeSt"));
    }
}
