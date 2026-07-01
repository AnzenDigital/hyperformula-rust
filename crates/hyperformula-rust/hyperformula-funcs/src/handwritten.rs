use hyperformula_core::{Error, Function, FunctionContext, Result, Value};

pub struct SumFunction;

impl Function for SumFunction {
    fn name(&self) -> &str {
        "SUM"
    }

    fn category(&self) -> &str {
        "Math"
    }

    fn description(&self) -> &str {
        "Returns the sum of numbers"
    }

    fn execute(&self, args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        let mut sum = 0.0;
        for arg in args {
            if let Some(n) = arg.as_number() {
                sum += n;
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

    fn aliases(&self) -> &[&str] {
        &["AVG"]
    }

    fn category(&self) -> &str {
        "Statistical"
    }

    fn description(&self) -> &str {
        "Returns the average (arithmetic mean) of the arguments"
    }

    fn execute(&self, args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        if args.is_empty() {
            return Err(Error::InvalidArgument(
                "AVERAGE requires at least one argument".to_string(),
            ));
        }

        let mut sum = 0.0;
        let mut count = 0;
        for arg in args {
            if let Some(n) = arg.as_number() {
                sum += n;
                count += 1;
            }
        }

        if count == 0 {
            return Err(Error::InvalidArgument(
                "AVERAGE requires at least one numeric argument".to_string(),
            ));
        }

        Ok(Value::Number(sum / count as f64))
    }
}

pub struct IfFunction;

impl Function for IfFunction {
    fn name(&self) -> &str {
        "IF"
    }

    fn category(&self) -> &str {
        "Logical"
    }

    fn description(&self) -> &str {
        "Returns one value if a condition is true and another value if it's false"
    }

    fn execute(&self, args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        if args.len() < 2 || args.len() > 3 {
            return Err(Error::InvalidArgument(
                "IF requires 2 or 3 arguments".to_string(),
            ));
        }

        let condition = args[0].as_boolean().ok_or_else(|| Error::TypeError {
            expected: "boolean".to_string(),
            actual: "other".to_string(),
        })?;

        if condition {
            Ok(args[1].clone())
        } else if args.len() == 3 {
            Ok(args[2].clone())
        } else {
            Ok(Value::Boolean(false))
        }
    }
}
