use crate::logical::IfFunction;
use crate::math::{AverageFunction, SumFunction};
use crate::Function;
use hyperformula_core::{Evaluator, FunctionContext, Function as CoreFunction};
use crate::generated;

pub fn register_all_functions(evaluator: &mut Evaluator) {
    register_function(evaluator, SumFunction);
    register_function(evaluator, AverageFunction);
    register_function(evaluator, IfFunction);
    
    register_generated_functions(evaluator);
}

fn register_generated_functions(evaluator: &mut Evaluator) {
    let ctx = FunctionContext::new();
    
    evaluator.register_function("CHOOSE", move |args| {
        generated::ChooseFunction.execute(args, &ctx)
    });
}

fn register_function<F: Function + 'static>(evaluator: &mut Evaluator, func: F) {
    let name = func.name().to_string();
    evaluator.register_function(&name, move |args| func.eval(args));
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyperformula_core::Value;

    #[test]
    fn test_register_all() {
        let mut evaluator = Evaluator::new();
        register_all_functions(&mut evaluator);

        assert!(evaluator.has_function("SUM"));
        assert!(evaluator.has_function("AVERAGE"));
        assert!(evaluator.has_function("IF"));

        let result = evaluator.call_function("SUM", &[Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(result.unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_all_functions_work() {
        let mut evaluator = Evaluator::new();
        register_all_functions(&mut evaluator);

        let sum_result = evaluator.call_function(
            "SUM",
            &[
                Value::Number(10.0),
                Value::Number(20.0),
                Value::Number(30.0),
            ],
        );
        assert_eq!(sum_result.unwrap(), Value::Number(60.0));

        let avg_result =
            evaluator.call_function("AVERAGE", &[Value::Number(10.0), Value::Number(20.0)]);
        assert_eq!(avg_result.unwrap(), Value::Number(15.0));

        let if_result = evaluator.call_function(
            "IF",
            &[
                Value::Bool(true),
                Value::String("yes".to_string()),
                Value::String("no".to_string()),
            ],
        );
        assert_eq!(if_result.unwrap(), Value::String("yes".to_string()));
    }
}
