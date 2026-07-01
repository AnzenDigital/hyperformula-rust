use hyperformula_core::{Evaluator, Value};
use hyperformula_funcs::register_all_functions;

fn setup() -> Evaluator {
    let mut evaluator = Evaluator::new();
    register_all_functions(&mut evaluator);
    evaluator
}

#[test]
fn test_average_basic() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[Value::Number(2.0), Value::Number(4.0), Value::Number(6.0)],
    );
    assert_eq!(result.unwrap(), Value::Number(4.0));
}

#[test]
fn test_average_single_value() {
    let evaluator = setup();
    let result = evaluator.call_function("AVERAGE", &[Value::Number(42.0)]);
    assert_eq!(result.unwrap(), Value::Number(42.0));
}

#[test]
fn test_average_empty_range() {
    let evaluator = setup();
    let result = evaluator.call_function("AVERAGE", &[Value::Range(vec![])]);
    assert!(result.is_err());
}

#[test]
fn test_average_with_empty_values() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[
            Value::Number(10.0),
            Value::Empty,
            Value::Number(20.0),
            Value::Empty,
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(15.0));
}

#[test]
fn test_average_with_booleans() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[Value::Number(8.0), Value::Bool(true), Value::Bool(false)],
    );
    assert_eq!(result.unwrap(), Value::Number(3.0));
}

#[test]
fn test_average_with_numeric_strings() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[Value::Number(10.0), Value::String("20".to_string())],
    );
    assert_eq!(result.unwrap(), Value::Number(15.0));
}

#[test]
fn test_average_ignores_non_numeric_strings() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[
            Value::Number(10.0),
            Value::String("hello".to_string()),
            Value::Number(20.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(15.0));
}

#[test]
fn test_average_with_range() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[Value::Range(vec![
            Value::Number(10.0),
            Value::Number(20.0),
            Value::Number(30.0),
        ])],
    );
    assert_eq!(result.unwrap(), Value::Number(20.0));
}

#[test]
fn test_average_error_propagation() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[
            Value::Number(10.0),
            Value::Error("#VALUE!".to_string()),
            Value::Number(20.0),
        ],
    );
    assert!(result.unwrap().is_error());
}

#[test]
fn test_average_negative_numbers() {
    let evaluator = setup();
    let result = evaluator.call_function("AVERAGE", &[Value::Number(10.0), Value::Number(-10.0)]);
    assert_eq!(result.unwrap(), Value::Number(0.0));
}

#[test]
fn test_average_only_empty_ignored() {
    let evaluator = setup();
    let result = evaluator.call_function("AVERAGE", &[Value::Empty, Value::Empty, Value::Empty]);
    assert!(result.is_err());
}

#[test]
fn test_average_only_non_numeric_strings() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "AVERAGE",
        &[
            Value::String("hello".to_string()),
            Value::String("world".to_string()),
        ],
    );
    assert!(result.is_err());
}
