use hyperformula_core::{Error, Value};

pub trait Function: Send + Sync {
    fn name(&self) -> &str;

    fn min_arity(&self) -> Option<usize> {
        None
    }

    fn max_arity(&self) -> Option<usize> {
        None
    }

    fn eval(&self, args: &[Value]) -> Result<Value, Error>;

    fn validate_arity(&self, args: &[Value]) -> Result<(), Error> {
        let arg_count = args.len();

        if let Some(min) = self.min_arity() {
            if arg_count < min {
                return Err(Error::ValueError(format!(
                    "{} expects at least {} arguments, got {}",
                    self.name(),
                    min,
                    arg_count
                )));
            }
        }

        if let Some(max) = self.max_arity() {
            if arg_count > max {
                return Err(Error::ValueError(format!(
                    "{} expects at most {} arguments, got {}",
                    self.name(),
                    max,
                    arg_count
                )));
            }
        }

        Ok(())
    }
}
