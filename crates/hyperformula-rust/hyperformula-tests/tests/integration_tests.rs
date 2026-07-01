use hyperformula_core::{Evaluator, Value};
use hyperformula_funcs::register_all_functions;

fn setup() -> Evaluator {
    let mut evaluator = Evaluator::new();
    register_all_functions(&mut evaluator);
    evaluator
}

#[test]
fn test_combined_sum_and_average() {
    let evaluator = setup();

    let data = vec![
        Value::Number(10.0),
        Value::Number(20.0),
        Value::Number(30.0),
    ];

    let sum_result = evaluator.call_function("SUM", &data);
    assert_eq!(sum_result.unwrap(), Value::Number(60.0));

    let avg_result = evaluator.call_function("AVERAGE", &data);
    assert_eq!(avg_result.unwrap(), Value::Number(20.0));
}

#[test]
fn test_if_with_sum() {
    let evaluator = setup();

    let sum_result = evaluator
        .call_function("SUM", &[Value::Number(10.0), Value::Number(20.0)])
        .unwrap();

    let if_result = evaluator.call_function(
        "IF",
        &[Value::Number(1.0), sum_result.clone(), Value::Number(0.0)],
    );

    assert_eq!(if_result.unwrap(), sum_result);
}

#[test]
fn test_if_with_average_condition() {
    let evaluator = setup();

    let avg_result = evaluator
        .call_function("AVERAGE", &[Value::Number(10.0), Value::Number(20.0)])
        .unwrap();

    let if_result = evaluator.call_function(
        "IF",
        &[
            avg_result,
            Value::String("high".to_string()),
            Value::String("low".to_string()),
        ],
    );

    assert_eq!(if_result.unwrap(), Value::String("high".to_string()));
}

#[test]
fn test_complex_range_operations() {
    let evaluator = setup();

    let range = Value::Range(vec![
        Value::Number(5.0),
        Value::Bool(true),
        Value::String("10".to_string()),
        Value::Empty,
        Value::Number(15.0),
    ]);

    let sum_result = evaluator.call_function("SUM", std::slice::from_ref(&range));
    assert_eq!(sum_result.unwrap(), Value::Number(31.0));

    let avg_result = evaluator.call_function("AVERAGE", std::slice::from_ref(&range));
    assert_eq!(avg_result.unwrap(), Value::Number(31.0 / 4.0));
}

#[test]
fn test_all_functions_registered() {
    let evaluator = setup();
    let functions = evaluator.list_functions();

    assert!(functions.contains(&"SUM".to_string()));
    assert!(functions.contains(&"AVERAGE".to_string()));
    assert!(functions.contains(&"IF".to_string()));
    assert!(functions.contains(&"CHOOSE".to_string()));
    assert_eq!(functions.len(), 4);
}

#[test]
fn test_function_case_insensitivity() {
    let evaluator = setup();

    let result1 = evaluator.call_function("sum", &[Value::Number(1.0), Value::Number(2.0)]);
    let result2 = evaluator.call_function("SUM", &[Value::Number(1.0), Value::Number(2.0)]);
    let result3 = evaluator.call_function("Sum", &[Value::Number(1.0), Value::Number(2.0)]);

    assert_eq!(result1.unwrap(), Value::Number(3.0));
    assert_eq!(result2.unwrap(), Value::Number(3.0));
    assert_eq!(result3.unwrap(), Value::Number(3.0));
}

#[test]
fn test_spreadsheet_scenario_1() {
    let evaluator = setup();

    let sales = vec![
        Value::Number(1000.0),
        Value::Number(1500.0),
        Value::Number(1200.0),
        Value::Number(800.0),
    ];

    let total = evaluator.call_function("SUM", &sales).unwrap();
    assert_eq!(total, Value::Number(4500.0));

    let average = evaluator.call_function("AVERAGE", &sales).unwrap();
    assert_eq!(average, Value::Number(1125.0));

    let check = evaluator
        .call_function(
            "IF",
            &[
                average,
                Value::String("Above target".to_string()),
                Value::String("Below target".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(check, Value::String("Above target".to_string()));
}

#[test]
fn test_spreadsheet_scenario_2() {
    let evaluator = setup();

    let scores = Value::Range(vec![
        Value::Number(85.0),
        Value::Number(92.0),
        Value::Number(78.0),
        Value::Number(88.0),
        Value::Empty,
    ]);

    let average = evaluator.call_function("AVERAGE", &[scores]).unwrap();

    let grade = evaluator
        .call_function(
            "IF",
            &[
                Value::Number((average.as_number().unwrap() - 80.0).signum()),
                Value::String("Pass".to_string()),
                Value::String("Fail".to_string()),
            ],
        )
        .unwrap();

    assert_eq!(grade, Value::String("Pass".to_string()));
}

#[test]
fn test_choose_basic() {
    let evaluator = setup();

    // CHOOSE(2, "Apple", "Banana", "Cherry") should return "Banana"
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(2.0),
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
            Value::String("Cherry".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("Banana".to_string()));
}

#[test]
fn test_choose_first_element() {
    let evaluator = setup();

    // CHOOSE(1, 10, 20, 30) should return 10
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(1.0),
            Value::Number(10.0),
            Value::Number(20.0),
            Value::Number(30.0),
        ],
    );
    assert_eq!(result.unwrap(), Value::Number(10.0));
}

#[test]
fn test_choose_last_element() {
    let evaluator = setup();

    // CHOOSE(3, "X", "Y", "Z") should return "Z"
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(3.0),
            Value::String("X".to_string()),
            Value::String("Y".to_string()),
            Value::String("Z".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("Z".to_string()));
}

#[test]
fn test_choose_index_out_of_bounds() {
    let evaluator = setup();

    // CHOOSE(5, "A", "B", "C") should error - index too large
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(5.0),
            Value::String("A".to_string()),
            Value::String("B".to_string()),
            Value::String("C".to_string()),
        ],
    );
    assert!(result.is_err());
}

#[test]
fn test_choose_index_zero() {
    let evaluator = setup();

    // CHOOSE(0, "A", "B") should error - index must be >= 1
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(0.0),
            Value::String("A".to_string()),
            Value::String("B".to_string()),
        ],
    );
    assert!(result.is_err());
}

#[test]
fn test_choose_negative_index() {
    let evaluator = setup();

    // CHOOSE(-1, "A", "B") should error - index must be >= 1
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(-1.0),
            Value::String("A".to_string()),
            Value::String("B".to_string()),
        ],
    );
    assert!(result.is_err());
}

#[test]
fn test_choose_insufficient_arguments() {
    let evaluator = setup();

    // CHOOSE(1) should error - need at least index + one value
    let result = evaluator.call_function("CHOOSE", &[Value::Number(1.0)]);
    assert!(result.is_err());
}

#[test]
fn test_choose_with_different_types() {
    let evaluator = setup();

    // CHOOSE can return different types
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(2.0),
            Value::Number(100.0),
            Value::Bool(true),
            Value::String("text".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::Bool(true));
}

#[test]
fn test_choose_with_fractional_index() {
    let evaluator = setup();

    // CHOOSE(2.7, "A", "B", "C") should use 2 (truncated)
    let result = evaluator.call_function(
        "CHOOSE",
        &[
            Value::Number(2.7),
            Value::String("A".to_string()),
            Value::String("B".to_string()),
            Value::String("C".to_string()),
        ],
    );
    assert_eq!(result.unwrap(), Value::String("B".to_string()));
}
