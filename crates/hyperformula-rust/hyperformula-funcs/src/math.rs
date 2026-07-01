use crate::Function;
use hyperformula_core::{Error, Value};

pub struct SumFunction;

impl Function for SumFunction {
    fn name(&self) -> &str {
        "SUM"
    }

    fn min_arity(&self) -> Option<usize> {
        Some(1)
    }

    fn eval(&self, args: &[Value]) -> Result<Value, Error> {
        self.validate_arity(args)?;

        let mut sum = 0.0;
        let flattened: Vec<Value> = args.iter().flat_map(|v| v.flatten()).collect();

        for value in flattened {
            match value {
                Value::Number(n) => sum += n,
                Value::Bool(true) => sum += 1.0,
                Value::Bool(false) => {}
                Value::Boolean(true) => sum += 1.0,
                Value::Boolean(false) => {}
                Value::String(s) => {
                    if let Ok(n) = s.trim().parse::<f64>() {
                        sum += n;
                    }
                }
                Value::Error(e) => return Ok(Value::Error(e)),
                Value::Empty => {}
                Value::Range(_) => unreachable!("Range should be flattened"),
                Value::Array(_) => unreachable!("Array should be flattened"),
            }
        }

        Ok(Value::Number(sum))
    }
}

pub struct AverageFunction;

impl Function for AverageFunction {
    fn name(&self) -> &str {
        "AVERAGE"
    }

    fn min_arity(&self) -> Option<usize> {
        Some(1)
    }

    fn eval(&self, args: &[Value]) -> Result<Value, Error> {
        self.validate_arity(args)?;

        let mut sum = 0.0;
        let mut count = 0;
        let flattened: Vec<Value> = args.iter().flat_map(|v| v.flatten()).collect();

        for value in flattened {
            match value {
                Value::Number(n) => {
                    sum += n;
                    count += 1;
                }
                Value::Bool(b) => {
                    sum += if b { 1.0 } else { 0.0 };
                    count += 1;
                }
                Value::Boolean(b) => {
                    sum += if b { 1.0 } else { 0.0 };
                    count += 1;
                }
                Value::String(s) => {
                    if let Ok(n) = s.trim().parse::<f64>() {
                        sum += n;
                        count += 1;
                    }
                }
                Value::Error(e) => return Ok(Value::Error(e)),
                Value::Empty => {}
                Value::Range(_) => unreachable!("Range should be flattened"),
                Value::Array(_) => unreachable!("Array should be flattened"),
            }
        }

        if count == 0 {
            return Err(Error::DivisionByZero);
        }

        Ok(Value::Number(sum / count as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        let sum = SumFunction;
        let result = sum.eval(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        assert_eq!(result.unwrap(), Value::Number(6.0));
    }

    #[test]
    fn test_sum_with_bools() {
        let sum = SumFunction;
        let result = sum.eval(&[Value::Number(10.0), Value::Bool(true), Value::Bool(false)]);
        assert_eq!(result.unwrap(), Value::Number(11.0));
    }

    #[test]
    fn test_sum_with_strings() {
        let sum = SumFunction;
        let result = sum.eval(&[
            Value::Number(5.0),
            Value::String("10".to_string()),
            Value::String("not a number".to_string()),
        ]);
        assert_eq!(result.unwrap(), Value::Number(15.0));
    }

    #[test]
    fn test_sum_empty_range() {
        let sum = SumFunction;
        let result = sum.eval(&[Value::Range(vec![])]);
        assert_eq!(result.unwrap(), Value::Number(0.0));
    }

    #[test]
    fn test_average_numbers() {
        let avg = AverageFunction;
        let result = avg.eval(&[Value::Number(2.0), Value::Number(4.0), Value::Number(6.0)]);
        assert_eq!(result.unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_average_empty() {
        let avg = AverageFunction;
        let result = avg.eval(&[Value::Range(vec![])]);
        assert!(result.is_err());
    }

    #[test]
    fn test_average_with_empty_values() {
        let avg = AverageFunction;
        let result = avg.eval(&[Value::Number(10.0), Value::Empty, Value::Number(20.0)]);
        assert_eq!(result.unwrap(), Value::Number(15.0));
    }
}
