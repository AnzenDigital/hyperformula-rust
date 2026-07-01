use hyperformula_core::{Evaluator, Value};
use hyperformula_funcs::register_all_functions;

fn setup() -> Evaluator {
    let mut evaluator = Evaluator::new();
    register_all_functions(&mut evaluator);
    evaluator
}

#[test]
fn test_if_true_condition() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Bool(true),
            Value::String("yes".to_string()),
            Value::String("no".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("yes".to_string()));
}

#[test]
fn test_if_false_condition() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Bool(false),
            Value::String("yes".to_string()),
            Value::String("no".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("no".to_string()));
}

#[test]
fn test_if_without_else_true() {
    let evaluator = setup();
    let result = evaluator.call_function("IF", &[Value::Bool(true), Value::Number(100.0)]);
    assert_eq!(result.unwrap(), Value::Number(100.0));
}

#[test]
fn test_if_without_else_false() {
    let evaluator = setup();
    let result = evaluator.call_function("IF", &[Value::Bool(false), Value::Number(100.0)]);
    assert_eq!(result.unwrap(), Value::Bool(false));
}

#[test]
fn test_if_numeric_condition_nonzero() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Number(5.0),
            Value::String("truthy".to_string()),
            Value::String("falsy".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("truthy".to_string()));
}

#[test]
fn test_if_numeric_condition_zero() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Number(0.0),
            Value::String("truthy".to_string()),
            Value::String("falsy".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("falsy".to_string()));
}

#[test]
fn test_if_string_condition_true() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::String("true".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(1.0));
}

#[test]
fn test_if_string_condition_false() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::String("false".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(0.0));
}

#[test]
fn test_if_empty_condition() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Empty,
            Value::String("yes".to_string()),
            Value::String("no".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("no".to_string()));
}

#[test]
fn test_if_error_propagation() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Error("#DIV/0!".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert!(result.unwrap().is_error());
}

#[test]
fn test_if_returns_different_types() {
    let evaluator = setup();

    let result = evaluator.call_function(
        "IF",
        &[
            Value::Bool(true),
            Value::Number(123.0),
            Value::String("text".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(123.0));

    let result = evaluator.call_function(
        "IF",
        &[
            Value::Bool(false),
            Value::Number(123.0),
            Value::String("text".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("text".to_string()));
}

#[test]
fn test_if_nested_values() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::Bool(true),
            Value::Range(vec![Value::Number(1.0), Value::Number(2.0)]),
            Value::Number(0.0),
        ],
    );
    assert!(matches!(result.unwrap(), Value::Range(_)));
}

#[test]
fn test_if_invalid_string_condition() {
    let evaluator = setup();
    let result = evaluator.call_function(
        "IF",
        &[
            Value::String("not a boolean".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert!(result.is_err());
}

#[test]
fn test_if_case_insensitive_bool_strings() {
    let evaluator = setup();

    let result = evaluator.call_function(
        "IF",
        &[
            Value::String("TRUE".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(1.0));

    let result = evaluator.call_function(
        "IF",
        &[
            Value::String("False".to_string()),
            Value::Number(1.0),
            Value::Number(0.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(0.0));
}
