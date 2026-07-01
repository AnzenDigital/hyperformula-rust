use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
    pub variadic: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSpec {
    pub name: String,
    pub aliases: Vec<String>,
    pub category: String,
    pub description: String,
    pub signature: Option<String>,
    pub parameters: Vec<FunctionParameter>,
    pub examples: Vec<String>,
    pub url: String,
    pub skipped: bool,
    pub skip_reason: Option<String>,
}

impl FunctionSpec {
    pub fn new(name: String) -> Self {
        Self {
            name,
            aliases: Vec::new(),
            category: String::from("General"),
            description: String::new(),
            signature: None,
            parameters: Vec::new(),
            examples: Vec::new(),
            url: String::new(),
            skipped: false,
            skip_reason: None,
        }
    }
}
