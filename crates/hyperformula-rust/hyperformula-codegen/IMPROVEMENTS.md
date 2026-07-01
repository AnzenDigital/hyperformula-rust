# Codegen Improvements for Module V (Lookup Functions)

## Summary

Enhanced the HyperFormula code generator to produce higher-quality stubs for lookup functions (VLOOKUP, HLOOKUP, INDEX, MATCH, CHOOSE) with detailed parameter information, validation code, and comprehensive documentation.

## Changes Made

### 1. Enhanced FunctionSpec Structure

Added parameter metadata to the function specification:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
    pub variadic: bool,
    pub description: String,
}
```

### 2. Hardcoded Parameter Information

Added comprehensive parameter information for all lookup functions:

- **VLOOKUP**: 3 required + 1 optional parameter
  - `lookup_value` (Any): The value to search for
  - `table_array` (Range): The table to search
  - `col_index_num` (Number): Column index (1-based)
  - `range_lookup` (Boolean, optional): Approximate/exact match

- **HLOOKUP**: Similar structure for horizontal lookup

- **INDEX**: 2 required + 1 optional parameter
  - `array` (Range): The data range
  - `row_num` (Number): Row index (1-based)
  - `column_num` (Number, optional): Column index (1-based)

- **MATCH**: 2 required + 1 optional parameter
  - `lookup_value` (Any): Value to find
  - `lookup_array` (Range): Range to search
  - `match_type` (Number, optional): Match mode (-1, 0, 1)

- **CHOOSE**: 2+ parameters with variadic value metadata
  - `index_num` (Number): Index to select (1-based)
  - `values` (Any, variadic): Values to choose from

### 3. Enhanced Code Generation

#### Before:
```rust
pub struct VlookupFunction;

impl Function for VlookupFunction {
    fn execute(&self, _args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        Err(Error::NotImplemented("VLOOKUP function not yet implemented".to_string()))
    }
}
```

#### After:
```rust
/// Looks up a value in the first column and returns a value in the same row
///
/// **Status**: Not implemented (stub)
///
/// **Category**: Lookup
/// **Documentation**: <https://hyperformula.handsontable.com/guide/built-in-functions.html>
///
/// **Signature**: `VLOOKUP(lookup_value, table_array, col_index_num, [range_lookup])`
///
/// # Parameters
/// - `lookup_value`: Any - The value to search for in the first column of the table
/// - `table_array`: Range - The table of data to search. Must be at least 2 columns wide
/// - `col_index_num`: Number - The column number in the table from which to return a value (1-based)
/// - `range_lookup`: Boolean - Optional. TRUE for approximate match (default), FALSE for exact match (optional)
///
/// # TODO
/// Implement this function according to the HyperFormula specification.
pub struct VlookupFunction;

impl Function for VlookupFunction {
    fn execute(&self, args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
        // Validate argument count (3 required, 1 optional)
        if args.len() < 3 || args.len() > 4 {
            return Err(Error::InvalidArgument(format!("Expected 3-4 arguments, got {}", args.len())));
        }

        // TODO: Extract and validate parameters:
        // let lookup_value = &args[0]; // lookup_value: Any
        // let table_array = &args[1]; // table_array: Range
        // let col_index_num = &args[2]; // col_index_num: Number
        // let range_lookup = args.get(3); // range_lookup: Boolean (optional)

        Err(Error::NotImplemented("VLOOKUP function not yet implemented".to_string()))
    }
}
```

### 4. Improved spec.json

The JSON specification now includes structured parameter information:

```json
{
  "name": "VLOOKUP",
  "signature": "VLOOKUP(lookup_value, table_array, col_index_num, [range_lookup])",
  "parameters": [
    {
      "name": "lookup_value",
      "param_type": "Any",
      "optional": false,
      "variadic": false,
      "description": "The value to search for in the first column of the table"
    },
    ...
  ]
}
```

## Benefits

1. **Better Documentation**: Function stubs now include complete parameter documentation
2. **Argument Validation**: Automatic validation of argument counts (required, optional, and variadic)
3. **Developer Guidance**: TODO comments show safe required, optional, and variadic parameter extraction
4. **Type Information**: Parameter types are clearly documented (Any, Range, Number, Boolean)
5. **Spec Tracking**: JSON spec includes all parameter metadata for reference

## Implementation Guide

When implementing a lookup function:

1. **Remove validation code** if you need custom validation
2. **Follow the TODO comments** to extract parameters
3. **Check parameter types** using the documented type information
4. **Handle optional parameters** properly (use `args.get(index)` or check length)
5. **Handle variadic parameters** as argument slices, for example `&args[1..]`
6. **Update the documentation** to reflect implementation status

## Example: Implementing VLOOKUP

```rust
fn execute(&self, args: &[Value], _ctx: &FunctionContext) -> Result<Value> {
    // Validate argument count (3 required, 1 optional)
    if args.len() < 3 || args.len() > 4 {
        return Err(Error::InvalidArgument(format!("Expected 3-4 arguments, got {}", args.len())));
    }

    // Extract parameters
    let lookup_value = &args[0];
    let table_array = match &args[1] {
        Value::Range(range) => range,
        _ => return Err(Error::TypeError { 
            expected: "Range".to_string(), 
            actual: "...".to_string() 
        }),
    };
    let col_index = args[2].to_number()?;
    let range_lookup = args.get(3)
        .and_then(|v| v.to_bool().ok())
        .unwrap_or(true);
    
    // Implement lookup logic...
    // ...
}
```

## Files Modified

- `hyperformula-codegen/src/spec.rs` - Added FunctionParameter struct
- `hyperformula-codegen/src/scraper.rs` - Added parameter population logic
- `hyperformula-codegen/src/generator.rs` - Enhanced code generation with validation and docs
- `hyperformula-funcs/src/generated.rs` - Regenerated with improved stubs
- `hyperformula-codegen/out/spec.json` - Updated with parameter metadata

## Testing

All tests pass successfully:
```bash
$ cargo build --workspace
   Compiling hyperformula-funcs v0.1.0
   Compiling hyperformula-tests v0.1.0
   Compiling hyperformula-cli v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s

$ cargo test --workspace
   test result: ok. 70 passed; 0 failed; 103 ignored; 0 measured
```

## Future Improvements

1. **Web scraping enhancement**: Extract parameter info from HyperFormula docs
2. **More functions**: Add parameter info for other function categories
3. **Type validation**: Generate type checking code for parameters
4. **Example generation**: Generate working examples in doc comments
5. **Return type info**: Document expected return types
