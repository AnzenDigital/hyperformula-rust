use hyperformula_core::{Evaluator, Value};
use hyperformula_funcs::register_all_functions;

fn setup() -> Evaluator {
    let mut evaluator = Evaluator::new();
    register_all_functions(&mut evaluator);
    evaluator
}

#[test]
fn test_sum_basic() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)],
    );
    assert_eq!(result.unwrap(), Value::Number(6.0));
}

#[test]
fn test_sum_single_value() {
    let evaluator = setup();
    let result = evaluator.call_function("SUM", &[Value::Number(42.0)]);
    assert_eq!(result.unwrap(), Value::Number(42.0));
}

#[test]
fn test_sum_empty_range() {
    let evaluator = setup();
    let result = evaluator.call_function("SUM", &[Value::Range(vec![])]);
    assert_eq!(result.unwrap(), Value::Number(0.0));
}

#[test]
fn test_sum_with_empty_values() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(10.0),
            Value::Empty,
            Value::Number(20.0),
            Value::Empty,
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(30.0));
}

#[test]
fn test_sum_with_booleans() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(5.0),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(7.0));
}

#[test]
fn test_sum_with_numeric_strings() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(10.0),
            Value::String("5".to_string()),
            Value::String("2.5".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(17.5));
}

#[test]
fn test_sum_ignores_non_numeric_strings() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(10.0),
            Value::String("hello".to_string()),
            Value::Number(20.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(30.0));
}

#[test]
fn test_sum_with_range() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(1.0),
            Value::Range(vec![
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ]),
            Value::Number(5.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(15.0));
}

#[test]
fn test_sum_nested_ranges() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[Value::Range(vec![
            Value::Number(1.0),
            Value::Range(vec![Value::Number(2.0), Value::Number(3.0)]),
            Value::Number(4.0),
        ])],
    );
    assert_eq!(result.unwrap(), Value::Number(10.0));
}

#[test]
fn test_sum_error_propagation() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(10.0),
            Value::Error("#DIV/0!".to_string()),
            Value::Number(20.0),
        ],
    );
    assert!(result.unwrap().is_error());
}

#[test]
fn test_sum_negative_numbers() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[
            Value::Number(10.0),
            Value::Number(-5.0),
            Value::Number(-3.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(2.0));
}

#[test]
fn test_sum_floating_point() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "SUM",
        &[Value::Number(0.1), Value::Number(0.2), Value::Number(0.3)],
    );
    let sum = result.unwrap().as_number().unwrap();
    assert!((sum - 0.6).abs() < 1e-10);
}
