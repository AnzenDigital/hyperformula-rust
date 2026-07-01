use crate::Function;
use hyperformula_core::{Error, Value};

pub struct IfFunction;

impl Function for IfFunction {
    fn name(&self) -> &str {
        "IF"
    }

    fn min_arity(&self) -> Option<usize> {
        Some(2)
    }

    fn max_arity(&self) -> Option<usize> {
        Some(3)
    }

    fn eval(&self, args: &[Value]) -> Result<Value, Error> {
        self.validate_arity(args)?;

        if args[0].is_error() {
            return Ok(args[0].clone());
        }

        let condition = match &args[0] {
            Value::Empty => false,
            _ => args[0].as_boolean().ok_or_else(|| Error::TypeError {
                expected: "boolean".to_string(),
                actual: "other".to_string(),
            })?,
        };

        if condition {
            Ok(args[1].clone())
        } else if args.len() == 3 {
            Ok(args[2].clone())
        } else {
            Ok(Value::Bool(false))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_true_condition() {
        let if_func = IfFunction;
        let result = if_func.eval(&[
            Value::Bool(true),
            Value::String("yes".to_string()),
            Value::String("no".to_string()),
        ]);
        assert_eq!(result.unwrap(), Value::String("yes".to_string()));
    }

    #[test]
    fn test_if_false_condition() {
        let if_func = IfFunction;
        let result = if_func.eval(&[
            Value::Bool(false),
            Value::String("yes".to_string()),
            Value::String("no".to_string()),
        ]);
        assert_eq!(result.unwrap(), Value::String("no".to_string()));
    }

    #[test]
    fn test_if_number_condition() {
        let if_func = IfFunction;
        let result = if_func.eval(&[Value::Number(1.0), Value::Number(10.0), Value::Number(20.0)]);
        assert_eq!(result.unwrap(), Value::Number(10.0));

        let result = if_func.eval(&[Value::Number(0.0), Value::Number(10.0), Value::Number(20.0)]);
        assert_eq!(result.unwrap(), Value::Number(20.0));
    }

    #[test]
    fn test_if_without_else() {
        let if_func = IfFunction;
        let result = if_func.eval(&[Value::Bool(false), Value::String("yes".to_string())]);
        assert_eq!(result.unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_if_string_condition() {
        let if_func = IfFunction;
        let result = if_func.eval(&[
            Value::String("true".to_string()),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        assert_eq!(result.unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_if_error_propagation() {
        let if_func = IfFunction;
        let result = if_func.eval(&[
            Value::Error("#DIV/0!".to_string()),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        assert!(result.unwrap().is_error());
    }
}
