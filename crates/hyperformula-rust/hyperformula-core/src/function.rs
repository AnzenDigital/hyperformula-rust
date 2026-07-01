use crate::{Result, Value};

pub struct FunctionContext {
    // Future: sheet references, cell coordinates, etc.
}

impl FunctionContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Function: Send + Sync {
    fn name(&self) -> &str;

    fn aliases(&self) -> &[&str] {
        &[]
    }

    fn category(&self) -> &str {
        "General"
    }

    fn description(&self) -> &str {
        ""
    }

    fn execute(&self, args: &[Value], ctx: &FunctionContext) -> Result<Value>;
}
