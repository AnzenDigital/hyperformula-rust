use hyperformula_core::Function as CoreFunction;
use std::collections::HashMap;
use std::sync::Arc;

mod function;
mod generated;
pub mod handwritten;
mod logical;
mod math;
pub mod registry;

pub use function::Function;
pub use registry::register_all_functions;

pub struct FunctionRegistry {
    functions: HashMap<String, Arc<dyn CoreFunction>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };

        registry.register_handwritten_functions();
        registry.register_generated_functions();

        registry
    }

    pub fn register(&mut self, func: Arc<dyn CoreFunction>) {
        let name = func.name().to_uppercase();
        self.functions.insert(name.clone(), func.clone());

        for alias in func.aliases() {
            let alias_upper = alias.to_uppercase();
            self.functions.insert(alias_upper, func.clone());
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn CoreFunction>> {
        self.functions.get(&name.to_uppercase()).cloned()
    }

    pub fn all_functions(&self) -> Vec<Arc<dyn CoreFunction>> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();

        for func in self.functions.values() {
            let name = func.name();
            if seen.insert(name) {
                result.push(func.clone());
            }
        }

        result
    }

    fn register_handwritten_functions(&mut self) {
        self.register(Arc::new(handwritten::SumFunction));
        self.register(Arc::new(handwritten::AverageFunction));
        self.register(Arc::new(handwritten::IfFunction));
    }

    fn register_generated_functions(&mut self) {
        generated::register_generated_functions(self);
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
