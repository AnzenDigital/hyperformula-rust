# HyperFormula Rust

A high-performance Rust implementation of a HyperFormula-compatible spreadsheet function engine.

## Overview

This workspace provides a modular, extensible spreadsheet formula evaluation engine written in Rust. It's designed to be compatible with HyperFormula/Excel semantics and provides a foundation for implementing the full suite of 400+ spreadsheet functions.

## Architecture

The workspace is organized into four crates:

### `hyperformula-core`

Core types and evaluation infrastructure:

- **Value enum**: Represents all spreadsheet value types (Number, String, Bool, Error, Range, Empty)
- **Evaluator**: Function registry and evaluation engine
- **Coercion**: Type conversion utilities following spreadsheet semantics
- **Error**: Comprehensive error types matching Excel error kinds

### `hyperformula-funcs`

Function implementations and registry:

- **Function trait**: Defines the interface for all spreadsheet functions (name, arity, eval)
- **Math functions**: SUM, AVERAGE
- **Logical functions**: IF
- **Registry**: Central registration system for all functions

### `hyperformula-tests`

Comprehensive test suite:

- Unit tests for each function
- Integration tests covering multiple functions
- Corner case coverage (empty ranges, error propagation, type coercion)
- Spreadsheet scenario tests

### `hyperformula-cli`

Interactive command-line interface:

- Evaluate formulas from command line: `cargo run -- "SUM(1,2,3)"`
- Interactive REPL mode for testing
- Supports all registered functions

## Getting Started

### Build the workspace

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### Use the CLI

```bash
# Evaluate a single expression
cargo run -p hyperformula-cli -- "SUM(1,2,3)"
# Output: 6

# Interactive mode
cargo run -p hyperformula-cli
> SUM(10, 20, 30)
60
> AVERAGE(5, 10, 15)
10
> IF(true, "yes", "no")
yes
> exit
```

## Current Functions

The following three starter functions are implemented with full Excel/HyperFormula semantics:

### SUM(number1, [number2], ...)

Sums all numeric arguments and ranges.

- Flattens ranges automatically
- Converts booleans: TRUE=1, FALSE=0
- Parses numeric strings
- Ignores non-numeric strings
- Propagates errors
- Empty values treated as 0

**Examples:**
```rust
SUM(1, 2, 3)                    // 6
SUM(10, true, false, true)      // 12
SUM(5, "10", "text")            // 15
```

### AVERAGE(number1, [number2], ...)

Calculates arithmetic mean of numeric arguments.

- Only counts numeric values (ignores text)
- Converts booleans to numbers
- Errors on empty input
- Propagates errors

**Examples:**
```rust
AVERAGE(10, 20, 30)             // 20
AVERAGE(10, "text", 20)         // 15
AVERAGE()                       // Error: DivByZero
```

### IF(condition, value_if_true, [value_if_false])

Returns one value if condition is true, another if false.

- Converts condition to boolean
- Numbers: 0=false, non-zero=true
- Strings: "true"/"false" (case-insensitive)
- Empty=false
- Propagates errors in condition
- Returns FALSE if value_if_false omitted

**Examples:**
```rust
IF(true, "yes", "no")           // "yes"
IF(0, "yes", "no")              // "no"
IF(5 > 3, 100, 200)             // 100 (when comparison implemented)
```

## Development Guide

### Adding New Functions

1. **Create function struct** in appropriate module (`math.rs`, `text.rs`, etc.):

```rust
pub struct YourFunction;

impl Function for YourFunction {
    fn name(&self) -> &str {
        "YOUR_FUNCTION"
    }
    
    fn min_arity(&self) -> Option<usize> {
        Some(1)  // minimum number of arguments
    }
    
    fn max_arity(&self) -> Option<usize> {
        Some(3)  // maximum number of arguments, None for variadic
    }
    
    fn eval(&self, args: &[Value]) -> Result<Value, Error> {
        self.validate_arity(args)?;
        
        // Implementation here
        // - Use value.flatten() for ranges
        // - Use Coercion trait for type conversion
        // - Propagate errors early
        
        Ok(Value::Number(result))
    }
}
```

2. **Register the function** in `registry.rs`:

```rust
register_function(evaluator, YourFunction);
```

3. **Add tests** in `hyperformula-tests/tests/`:

```rust
#[test]
fn test_your_function() {
    let evaluator = setup();
    let result = evaluator.call_function("YOUR_FUNCTION", &[...]);
    assert_eq!(result.unwrap(), expected);
}
```

### Coding Standards

- **Error handling**: Use `Result<Value, Error>` for all function returns
- **Type coercion**: Use `Coercion` trait methods, not manual conversion
- **Range handling**: Always flatten ranges with `value.flatten()`
- **Error propagation**: Check for errors early and return immediately
- **Testing**: Every function needs comprehensive unit tests covering:
  - Happy path with typical inputs
  - Edge cases (empty, zero, negative)
  - Type coercion (strings, booleans, mixed)
  - Error propagation
  - Range handling

### Performance Considerations

- Functions should avoid allocations where possible
- Use iterators instead of collecting intermediate results
- Consider using `SmallVec` for common cases (TODO)
- Profile before optimizing (TODO: add benchmarks)

## Roadmap: Porting 400+ Functions

See [PLAN.md](./PLAN.md) for detailed roadmap.

### Priority Categories

1. **Math & Statistics** (40 functions): Basic math, trigonometry, statistical
2. **Logical** (10 functions): AND, OR, NOT, XOR, conditionals
3. **Text** (30 functions): String manipulation, formatting
4. **Date & Time** (25 functions): Date arithmetic, formatting
5. **Lookup & Reference** (15 functions): VLOOKUP, INDEX, MATCH
6. **Financial** (50 functions): NPV, IRR, PMT
7. **Engineering** (40 functions): Complex numbers, conversions
8. **Information** (20 functions): Type checking, error handling
9. **Database** (12 functions): Aggregation with criteria
10. **Array & Advanced** (remaining): Matrix operations, dynamic arrays

### Testing Strategy

- **Unit tests**: Every function, every edge case
- **Integration tests**: Multi-function scenarios
- **Property tests**: Randomized testing for mathematical properties (TODO)
- **Excel compatibility tests**: Compare against Excel output (TODO)
- **Performance benchmarks**: Ensure sub-microsecond evaluation (TODO)

## Benchmarks

Basic performance characteristics (TODO: add comprehensive benchmarks):

```
SUM(1000 numbers):     ~X μs
AVERAGE(1000 numbers): ~X μs
Nested IF (depth 10):  ~X μs
```

## Contributing

When contributing new functions:

1. Follow the Excel/HyperFormula specification exactly
2. Add comprehensive tests including edge cases
3. Update this README with function documentation
4. Run `cargo fmt` and `cargo clippy` before committing
5. Ensure all tests pass with `cargo test`

## License

MIT License - See LICENSE file for details

## References

- [HyperFormula Documentation](https://hyperformula.handsontable.com/)
- [Excel Functions Reference](https://support.microsoft.com/en-us/office/excel-functions-alphabetical-b3944572-255d-4efb-bb96-c6d90033e188)
- [OpenFormula Specification](https://docs.oasis-open.org/office/v1.2/os/OpenDocument-v1.2-os-part2.html)
