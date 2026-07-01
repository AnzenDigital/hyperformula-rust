use hyperformula_core::{FunctionContext, Value};
use hyperformula_funcs::FunctionRegistry;

#[test]
fn test_sum_basic() {
    let registry = FunctionRegistry::new();
    let sum_func = registry.get("SUM").unwrap();
    let ctx = FunctionContext::new();

    let args = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    let result = sum_func.execute(&args, &ctx).unwrap();

    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_average_basic() {
    let registry = FunctionRegistry::new();
    let avg_func = registry.get("AVERAGE").unwrap();
    let ctx = FunctionContext::new();

    let args = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    let result = avg_func.execute(&args, &ctx).unwrap();

    assert_eq!(result, Value::Number(2.0));
}

#[test]
fn test_if_basic() {
    let registry = FunctionRegistry::new();
    let if_func = registry.get("IF").unwrap();
    let ctx = FunctionContext::new();

    let args = vec![Value::Boolean(true), Value::Number(1.0), Value::Number(2.0)];
    let result = if_func.execute(&args, &ctx).unwrap();

    assert_eq!(result, Value::Number(1.0));
}
