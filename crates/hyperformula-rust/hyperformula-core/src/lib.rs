pub mod error;
pub mod evaluator;
pub mod function;
pub mod value;

pub use error::{Error, Result};
pub use evaluator::Evaluator;
pub use function::{Function, FunctionContext};
pub use value::Value;
