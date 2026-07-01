# HyperFormula Code Generator

Automated code generation tool that scrapes the [HyperFormula built-in functions specification](https://hyperformula.handsontable.com/guide/built-in-functions.html) and generates Rust function stubs and test skeletons.

## Overview

This tool accelerates the development of the hyperformula-rust implementation by:

1. Fetching function metadata from the official HyperFormula documentation
2. Generating compilable Rust function stubs that implement the `Function` trait
3. Creating test skeletons for each function
4. Producing a JSON specification for review and version control

## Installation

The codegen tool is part of the hyperformula-rust workspace:

```bash
cd crates/hyperformula-rust/hyperformula-codegen
cargo build --release
```

## Usage

### Basic Usage

Run the generator with default settings:

```bash
cargo run --bin hyperformula-codegen
```

This will:
- Fetch the function specification from the default URL
- Generate function stubs in `../hyperformula-funcs/src/generated.rs`
- Generate test skeletons in `../hyperformula-tests/src/generated_tests.rs`
- Write the specification to `./out/spec.json`

### Command-Line Options

```bash
cargo run --bin hyperformula-codegen -- \
  --source-url <URL> \
  --out-dir <PATH> \
  --tests-out <PATH> \
  --spec-out <PATH> \
  --force
```

**Options:**

- `--source-url` - URL to fetch the HyperFormula documentation (default: official docs)
- `--out-dir` - Output directory for generated function stubs (default: `../hyperformula-funcs/src`)
- `--tests-out` - Output directory for generated test skeletons (default: `../hyperformula-tests/src`)
- `--spec-out` - Output directory for the JSON specification (default: `./out`)
- `--force` - Overwrite existing files without prompting

## Output Files

### 1. Function Stubs (`generated.rs`)

Generated Rust code that includes:

- **Struct definitions** for each function (e.g., `AbsFunction`, `SumFunction`)
- **Function trait implementation** with:
  - `name()` - Function name (e.g., "ABS")
  - `aliases()` - Alternative names (if any)
  - `category()` - Function category (Math, Statistical, etc.)
  - `description()` - Brief description from the spec
  - `execute()` - Returns `NotImplemented` error until implemented
- **Registration function** - `register_generated_functions()` to add all functions to the registry

Example generated stub:

```rust
/// Returns the absolute value of a number
///
/// **Status**: Not implemented (stub)
///
/// **Category**: Math
/// **Documentation**: <https://hyperformula.handsontable.com/guide/built-in-functions.html>
///
/// # TODO
/// Implement this function according to the HyperFormula specification.
pub struct AbsFunction;

impl Function for AbsFunction {
    fn name(&self) -> &str {
        "ABS"
    }

    fn category(&self) -> &str {
        "Math"
    }

    fn description(&self) -> &str {
        "Returns the absolute value of a number"
    }

    fn execute(&self, _args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        Err(Error::NotImplemented("ABS function not yet implemented".to_string()))
    }
}
```

### 2. Test Skeletons (`generated_tests.rs`)

Generated test stubs marked with `#[ignore]` to prevent CI failures:

```rust
/// Test for ABS function
///
/// # TODO
/// Implement test cases according to the HyperFormula specification.
/// Documentation: <https://hyperformula.handsontable.com/guide/built-in-functions.html>
#[test]
#[ignore] // Remove this attribute when implementing the function
fn test_generated_abs() {
    let registry = FunctionRegistry::new();
    let func = registry.get("ABS").expect("Function should be registered");
    let ctx = FunctionContext::new();

    // TODO: Add test cases
    // Example:
    // let args = vec![Value::Number(-5.0)];
    // let result = func.execute(&args, &ctx);
    // assert_eq!(result, Ok(Value::Number(5.0)));

    // For now, just verify the function is registered
    assert_eq!(func.name(), "ABS");
}
```

### 3. Specification JSON (`spec.json`)

JSON metadata for all discovered functions:

```json
[
  {
    "name": "ABS",
    "aliases": [],
    "category": "Math",
    "description": "Returns the absolute value of a number",
    "signature": null,
    "examples": [],
    "url": "https://hyperformula.handsontable.com/guide/built-in-functions.html",
    "skipped": false,
    "skip_reason": null
  },
  ...
]
```

**Skipped functions** (already implemented in `handwritten.rs`) are marked with:
```json
{
  "name": "SUM",
  "skipped": true,
  "skip_reason": "Already implemented in handwritten.rs"
}
```

## Developer Workflow

### 1. Generate Code

Run the generator to create/update stubs:

```bash
cd crates/hyperformula-rust/hyperformula-codegen
cargo run --bin hyperformula-codegen
```

### 2. Review Generated Files

Check the output:

```bash
# Review the spec
cat out/spec.json | jq '.[] | {name, category, description}'

# Count generated functions
cat out/spec.json | jq '[.[] | select(.skipped == false)] | length'

# List skipped functions
cat out/spec.json | jq '.[] | select(.skipped == true) | .name'
```

### 3. Verify Compilation

Ensure generated code compiles:

```bash
cd ../../..  # Back to workspace root
cargo build --workspace
cargo test --workspace
```

All generated tests will be ignored by default. The handwritten tests should still pass.

### 4. Implement Functions

To implement a function:

1. Open `crates/hyperformula-rust/hyperformula-funcs/src/generated.rs`
2. Find the function stub (e.g., `AbsFunction`)
3. Replace the `execute()` method body with your implementation
4. Open `crates/hyperformula-rust/hyperformula-tests/src/generated_tests.rs`
5. Find the corresponding test (e.g., `test_generated_abs`)
6. Remove the `#[ignore]` attribute
7. Add real test cases
8. Run the tests: `cargo test`

### 5. Regenerate When Needed

If the HyperFormula documentation changes or you want to refresh the stubs:

```bash
cargo run --bin hyperformula-codegen
```

The generator is **idempotent** and will completely overwrite the generated files. Any custom implementations should be moved to `handwritten.rs` before regenerating.

## Architecture Notes

### Scraping Strategy

The scraper attempts multiple CSS selectors to extract function information:
1. Article headings with IDs (`article h3[id]`)
2. Function entry containers (`.function-entry`)
3. Headings containing "function" keyword

If web scraping fails or returns no results, the tool falls back to a hardcoded list of common spreadsheet functions (~100 functions covering Math, Statistical, Logical, Text, Date, Lookup, and Information categories).

### Skipping Handwritten Functions

The generator checks a hardcoded list of handwritten functions (`SUM`, `AVERAGE`, `IF`) and marks them as skipped in the spec. These functions are not regenerated to preserve manual implementations.

To add more handwritten functions:
1. Update `HANDWRITTEN_FUNCTIONS` in `src/generator.rs`
2. Regenerate to update the spec

### Error Handling

Generated stubs return `Error::NotImplemented` with a descriptive message. This ensures:
- Generated code compiles successfully
- Runtime attempts to call unimplemented functions produce clear errors
- Developers can easily find TODOs by searching for "NotImplemented"

## Troubleshooting

### Generator fails to fetch documentation

If the network request fails:
```
Error: Failed to fetch HTML from source URL
```

Solution: Check your internet connection or use a cached/local HTML file:
```bash
curl -o spec.html https://hyperformula.handsontable.com/guide/built-in-functions.html
cargo run --bin hyperformula-codegen -- --source-url file://$(pwd)/spec.html
```

### No functions found during scraping

If the HTML structure has changed:
```
Found 0 functions
```

Solution: The generator will automatically fall back to a common function list. To fix scraping:
1. Inspect the HTML structure
2. Update selectors in `src/scraper.rs`
3. Run the generator again

### Build errors after regeneration

If the workspace fails to compile after regeneration:
```
error[E0277]: the trait bound `SomeFunction: Function` is not satisfied
```

Solution: This indicates the `Function` trait signature changed. Update `src/generator.rs` to match the current trait definition.

## Contributing

To extend the code generator:

- **Add new metadata fields**: Update `FunctionSpec` in `src/spec.rs`
- **Improve scraping**: Modify `src/scraper.rs` to extract more information (signatures, examples)
- **Customize output**: Edit template generation in `src/generator.rs`

## See Also

- [PLAN_codegen.md](../../../docs/PLAN_codegen.md) - Strategy for batch-porting 400+ functions
- [hyperformula-funcs](../hyperformula-funcs/) - Function implementation crate
- [hyperformula-tests](../hyperformula-tests/) - Test suite
