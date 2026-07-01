# HyperFormula Rust: Implementation Plan

## Executive Summary

This document outlines the strategy for porting the full suite of 400+ spreadsheet functions from HyperFormula to Rust, building on the foundation of the three starter functions (SUM, AVERAGE, IF).

## Current State

### Completed ✓

- **Core infrastructure**: Value types, Evaluator, Coercion, Error handling
- **Function trait**: Extensible interface for all functions
- **Registry system**: Dynamic function registration
- **3 starter functions**: SUM, AVERAGE, IF with comprehensive tests
- **CLI tool**: Interactive testing interface
- **Test framework**: Unit, integration, and scenario tests

### Architecture Strengths

- **Type safety**: Rust's type system prevents entire classes of runtime errors
- **Performance**: Zero-cost abstractions, no GC overhead
- **Modularity**: Clean separation of concerns across crates
- **Extensibility**: New functions are simple to add via the Function trait

## Function Inventory

Based on HyperFormula and Excel compatibility, we need to implement approximately 400+ functions across 10 categories:

### 1. Math & Trigonometry (40 functions)

**Priority: HIGH** - Most commonly used in spreadsheets

Basic Math:
- ABS, CEILING, FLOOR, ROUND, ROUNDDOWN, ROUNDUP, TRUNC
- MOD, QUOTIENT, SIGN, POWER, SQRT, EXP, LN, LOG, LOG10
- GCD, LCM, MROUND
- SUMIF, SUMIFS, SUMSQ, SUMPRODUCT

Trigonometry:
- SIN, COS, TAN, ASIN, ACOS, ATAN, ATAN2
- SINH, COSH, TANH, ASINH, ACOSH, ATANH
- DEGREES, RADIANS, PI

Random:
- RAND, RANDBETWEEN

**Implementation notes:**
- Most delegate to `std::f64` methods
- SUMIF/SUMIFS require criteria evaluation (implement separately)
- Consider using const generics for rounding modes

### 2. Statistical Functions (60 functions)

**Priority: HIGH** - Core data analysis

Descriptive:
- COUNT, COUNTA, COUNTBLANK, COUNTIF, COUNTIFS
- MIN, MAX, MINA, MAXA
- MEDIAN, MODE.SNGL, MODE.MULT
- VAR.S, VAR.P, STDEV.S, STDEV.P
- QUARTILE, PERCENTILE, PERCENTRANK

Advanced:
- CORREL, COVARIANCE.S, COVARIANCE.P
- FORECAST, TREND, GROWTH
- LINEST, LOGEST
- STANDARDIZE, ZSCORE
- SKEW, KURT

Distributions:
- NORM.DIST, NORM.INV, NORM.S.DIST, NORM.S.INV
- T.DIST, T.INV, CHISQ.DIST, F.DIST
- BINOM.DIST, POISSON.DIST

**Implementation notes:**
- Partner with `statrs` crate for distributions
- Implement streaming algorithms for mean/variance (Welford's)
- Consider lazy evaluation for large datasets

### 3. Logical Functions (10 functions)

**Priority: HIGH** - Essential control flow

- AND, OR, NOT, XOR
- TRUE, FALSE
- IFERROR, IFNA
- IFS (multi-condition IF)
- SWITCH

**Implementation notes:**
- Short-circuit evaluation for AND/OR
- IFERROR/IFNA are error-handling wrappers
- IFS is syntactic sugar over nested IFs

### 4. Text Functions (30 functions)

**Priority: MEDIUM** - String manipulation

Basic:
- CONCATENATE, CONCAT, TEXTJOIN
- LEFT, RIGHT, MID, LEN
- FIND, SEARCH, REPLACE, SUBSTITUTE
- UPPER, LOWER, PROPER, TRIM, CLEAN
- EXACT, TEXT, VALUE

Advanced:
- CHAR, CODE, UNICODE, UNICHAR
- FIXED, DOLLAR
- REPT, REVERSE (non-standard but useful)
- SPLIT (array function)

**Implementation notes:**
- UTF-8 aware: use `chars()` not bytes
- TEXT function needs format code parser (mini language)
- Consider regex support for SEARCH (optional)

### 5. Date & Time Functions (25 functions)

**Priority: MEDIUM** - Common in business apps

Date:
- DATE, DATEVALUE, DAY, MONTH, YEAR
- TODAY, NOW
- EDATE, EOMONTH, WORKDAY, NETWORKDAYS
- DATEDIF
- WEEKDAY, WEEKNUM, ISOWEEKNUM

Time:
- TIME, TIMEVALUE, HOUR, MINUTE, SECOND

**Implementation notes:**
- Partner with `chrono` crate for date/time handling
- Excel stores dates as days since 1900-01-01 (with leap year bug)
- Need timezone handling for NOW()
- WORKDAY needs holiday calendar support

### 6. Lookup & Reference (15 functions)

**Priority: HIGH** - Critical for data manipulation

- VLOOKUP, HLOOKUP, XLOOKUP
- INDEX, MATCH
- CHOOSE, INDIRECT, OFFSET
- ROW, COLUMN, ROWS, COLUMNS
- AREAS, ADDRESS

**Implementation notes:**
- INDIRECT requires parsing cell references (A1, R1C1 notation)
- XLOOKUP is modern replacement for VLOOKUP
- INDEX+MATCH combo is very common pattern
- Consider caching for INDIRECT (expensive to re-parse)

### 7. Financial Functions (50 functions)

**Priority: MEDIUM** - Specialized but important

Time Value:
- PV, FV, NPV, IRR, XIRR
- PMT, PPMT, IPMT, NPER, RATE
- MIRR

Depreciation:
- SLN, DDB, SYD, VDB

Securities:
- PRICE, YIELD, DURATION, MDURATION
- ACCRINT, COUPDAYBS, COUPNUM

**Implementation notes:**
- Partner with financial math library or implement algorithms
- IRR/XIRR require iterative root-finding (Newton-Raphson)
- Most are complex but well-specified formulas
- Comprehensive test suite against Excel essential

### 8. Information Functions (20 functions)

**Priority: LOW** - Utility functions

Type checking:
- ISBLANK, ISERR, ISERROR, ISNA
- ISLOGICAL, ISNONTEXT, ISNUMBER, ISTEXT
- ISREF, ISFORMULA

System:
- CELL, INFO, TYPE, N, NA
- ERROR.TYPE, SHEET, SHEETS

**Implementation notes:**
- Most are trivial pattern matches on Value enum
- CELL/INFO may need evaluator context
- SHEET functions need workbook concept

### 9. Database Functions (12 functions)

**Priority: LOW** - Specialized

- DSUM, DAVERAGE, DCOUNT, DCOUNTA
- DMAX, DMIN, DPRODUCT
- DSTDEV, DSTDEVP, DVAR, DVARP
- DGET

**Implementation notes:**
- All follow same pattern: filter by criteria, then aggregate
- Share criteria evaluation logic with SUMIF/COUNTIF
- Input is structured database range

### 10. Engineering Functions (40 functions)

**Priority: LOW** - Specialized technical

Number systems:
- BIN2DEC, BIN2HEX, BIN2OCT
- DEC2BIN, DEC2HEX, DEC2OCT
- HEX2BIN, HEX2DEC, HEX2OCT
- OCT2BIN, OCT2DEC, OCT2HEX

Complex numbers:
- COMPLEX, IMREAL, IMAGINARY, IMABS, IMARGUMENT
- IMADD, IMSUB, IMMULT, IMDIV, IMPRODUCT
- IMPOWER, IMSQRT, IMEXP, IMLN, IMLOG10, IMLOG2
- IMSIN, IMCOS, IMTAN, etc.

Bitwise:
- BITAND, BITOR, BITXOR, BITLSHIFT, BITRSHIFT

Other:
- DELTA, GESTEP, ERF, ERFC, BESSELJ, BESSELK

**Implementation notes:**
- Complex number struct needed (or use `num-complex` crate)
- Bessel functions: use `special` crate
- Error functions: use `libm` or `statrs`

## Implementation Strategy

### Phase 1: Foundation Expansion (Weeks 1-2)

**Goal**: Establish patterns for each function category

1. **Math & Trig** (10 functions): ABS, ROUND, SQRT, POWER, SIN, COS, MOD, RAND, PI, SUMIF
2. **Logical** (5 functions): AND, OR, NOT, IFERROR, IFS
3. **Text** (5 functions): CONCATENATE, LEFT, LEN, UPPER, FIND
4. **Information** (5 functions): ISBLANK, ISNUMBER, ISTEXT, ISERROR, TYPE

**Deliverables**:
- Module organization pattern (`math.rs`, `text.rs`, etc.)
- Shared utility functions (criteria parsing, range filtering)
- Extended test patterns for each category

### Phase 2: High-Priority Functions (Weeks 3-6)

**Goal**: Cover 80% of real-world usage

1. Math & Trig: Complete all 40 functions
2. Statistical: Core 20 functions (COUNT variants, MIN/MAX, MEDIAN, STDEV)
3. Logical: All 10 functions
4. Lookup: VLOOKUP, INDEX, MATCH, XLOOKUP
5. Text: All 30 functions

**Deliverables**:
- ~100 functions implemented
- Criteria evaluation engine (for SUMIF, COUNTIF, etc.)
- Date/time foundation (chrono integration)

### Phase 3: Specialized Functions (Weeks 7-10)

**Goal**: Complete coverage of all categories

1. Date & Time: All 25 functions
2. Financial: Core 20 functions (PV, FV, PMT, NPV, IRR)
3. Statistical: Advanced 40 functions (distributions, regression)
4. Lookup: Complete remaining functions
5. Database: All 12 functions

**Deliverables**:
- ~250 functions total
- Financial math library integration
- Statistical distributions (statrs integration)

### Phase 4: Edge Cases & Compatibility (Weeks 11-12)

**Goal**: Complete 400+ functions, Excel compatibility

1. Engineering: All 40 functions
2. Array functions: SORT, FILTER, UNIQUE, SEQUENCE
3. Remaining specialized functions
4. Excel compatibility testing

**Deliverables**:
- 400+ functions complete
- Excel compatibility test suite (compare outputs)
- Performance benchmarks
- Documentation for all functions

## Automation Strategy

### Function Generation from Spec

Many functions follow predictable patterns. Consider building generators:

1. **Simple math wrappers**: `fn(x: f64) -> f64` maps to spreadsheet function
2. **Aggregation functions**: Template for COUNT/SUM/AVERAGE pattern
3. **Type checking**: Auto-generate IS* functions from enum variants
4. **Number system converters**: Generate all BIN/DEC/HEX/OCT combinations

**Implementation**:
```rust
// macro or build.rs script
generate_type_check_functions! {
    ISNUMBER => Value::Number,
    ISTEXT => Value::String,
    ISLOGICAL => Value::Bool,
    ISERROR => Value::Error,
}
```

### Test Generation

Excel compatibility tests can be partially automated:

1. **Excel workbook as test fixture**: Define test cases in .xlsx
2. **Extractor**: Read expected outputs from Excel
3. **Test generator**: Create Rust tests comparing our output to Excel's
4. **Property-based tests**: Use `proptest` for mathematical properties

## Testing Matrix

### Unit Tests (per function)

- ✓ Basic happy path
- ✓ Edge cases (zero, negative, empty)
- ✓ Type coercion (string to number, etc.)
- ✓ Error propagation
- ✓ Range handling (flat, nested, empty)
- ✓ Boundary values (max/min)

### Integration Tests

- Multi-function formulas
- Circular dependency detection (TODO: not yet supported)
- Performance on large datasets
- Memory usage patterns

### Compatibility Tests

- Excel test workbook with known outputs
- OpenFormula spec compliance
- HyperFormula parity

### Property-Based Tests

Mathematical properties that should always hold:
- `SUM(a,b) == SUM(b,a)` (commutative)
- `AVERAGE(range) == SUM(range) / COUNT(range)`
- `ABS(x) >= 0`
- `SQRT(x)^2 ≈ x` (for x >= 0)

**Implementation**:
```rust
#[cfg(test)]
mod properties {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn sum_commutative(a: f64, b: f64) {
            let sum1 = call("SUM", &[a.into(), b.into()]);
            let sum2 = call("SUM", &[b.into(), a.into()]);
            prop_assert_eq!(sum1, sum2);
        }
    }
}
```

## Function Signature Conventions

All functions follow these conventions:

### Naming

- Function names: UPPERCASE (as in Excel)
- Rust structs: PascalCase + "Function" suffix
- Module files: lowercase (math.rs, text.rs)

### Arity

- Fixed arity: Set both `min_arity` and `max_arity`
- Variable arity: Set `min_arity` only, `max_arity = None`
- Optional args: Set different min/max values

### Argument Processing

1. **Validate arity** first: `self.validate_arity(args)?`
2. **Check for errors** in args: propagate immediately
3. **Flatten ranges** if function accepts ranges: `value.flatten()`
4. **Coerce types** using Coercion trait: `value.to_number()?`
5. **Process** and return result

### Error Handling

- Use `?` operator for early returns
- Propagate errors from arguments: `if arg.is_error() { return Ok(arg.clone()); }`
- Return appropriate error type: `Error::Value`, `Error::DivByZero`, etc.
- Never panic: all errors must be `Result::Err`

### Example Template

```rust
pub struct NewFunction;

impl Function for NewFunction {
    fn name(&self) -> &str {
        "NEWFUNCTION"
    }
    
    fn min_arity(&self) -> Option<usize> {
        Some(2)
    }
    
    fn max_arity(&self) -> Option<usize> {
        Some(3)
    }
    
    fn eval(&self, args: &[Value]) -> Result<Value, Error> {
        self.validate_arity(args)?;
        
        // Error propagation
        for arg in args {
            if let Value::Error(e) = arg {
                return Ok(Value::Error(e.clone()));
            }
        }
        
        // Type coercion
        let x = args[0].to_number()?;
        let y = args[1].to_number()?;
        let z = if args.len() > 2 {
            args[2].to_number()?
        } else {
            0.0
        };
        
        // Implementation
        let result = compute(x, y, z);
        
        Ok(Value::Number(result))
    }
}
```

## Error Propagation Strategy

Excel error types and their usage:

- **#DIV/0!** (`ErrorKind::DivByZero`): Division by zero
- **#VALUE!** (`ErrorKind::Value`): Wrong type of argument
- **#REF!** (`ErrorKind::Ref`): Invalid cell reference
- **#NAME?** (`ErrorKind::Name`): Unrecognized function name
- **#NUM!** (`ErrorKind::Num`): Invalid numeric value (e.g., SQRT(-1))
- **#N/A** (`ErrorKind::NA`): Value not available
- **#NULL!** (`ErrorKind::NullValue`): Invalid range intersection

Functions must:
1. Check arguments for errors first
2. Return errors immediately (propagate)
3. Generate appropriate error type for invalid operations
4. Never mask errors with default values

## Performance Optimization Plan

### Phase 1: Measure

- Benchmark each function category
- Profile with real-world workbooks
- Identify bottlenecks

### Phase 2: Optimize Hot Paths

- **Range flattening**: Consider lazy evaluation
- **String operations**: Reduce allocations with `Cow<str>`
- **Numeric operations**: SIMD for bulk operations (e.g., SUM of 10k numbers)
- **Lookup functions**: Use binary search, hash maps where appropriate

### Phase 3: Advanced

- **Caching**: Memoize expensive functions (IRR, INDIRECT)
- **Parallel evaluation**: Rayon for independent cells
- **JIT compilation**: Consider compiling formula AST to native code (future)

### Performance Goals

- Simple functions (SUM, IF): <100ns
- Complex functions (VLOOKUP): <10μs
- Statistical functions: <100μs
- Financial iterative (IRR): <1ms

## Dependencies Strategy

Prefer standard library, add dependencies when clear value:

**Approved**:
- `thiserror`: Error handling (already in use)
- `serde`: Serialization (already in use)
- `chrono`: Date/time (for date functions)
- `statrs`: Statistical distributions
- `num-complex`: Complex numbers (engineering functions)

**Consider**:
- `libm`: Math functions (if std is insufficient)
- `rayon`: Parallelism (for large datasets)
- `proptest`: Property-based testing

**Avoid**:
- Heavy dependencies with large transitive trees
- Unmaintained crates
- Dependencies that duplicate std functionality

## Documentation Standards

Each function needs:

1. **Docstring** with Excel-compatible description
2. **Syntax** showing arguments
3. **Examples** (at least 2-3)
4. **Edge cases** documented
5. **Compatibility notes** if different from Excel

Example:
```rust
/// Rounds a number to a specified number of digits.
///
/// Syntax: `ROUND(number, num_digits)`
///
/// Examples:
/// - `ROUND(2.15, 1)` returns `2.2`
/// - `ROUND(2.149, 1)` returns `2.1`
/// - `ROUND(-1.475, 2)` returns `-1.48`
///
/// Compatibility:
/// - Follows Excel's "round half away from zero" method
/// - Different from Rust's default "round half to even"
pub struct RoundFunction;
```

## Open Questions / TODOs

1. **Formula parsing**: Do we need a full formula parser? Or just function calls?
   - Current CLI only supports function calls
   - Full parser needed for: operators (+, -, *, /), cell references (A1:B10), precedence
   - Consider: `pest`, `nom`, or hand-written recursive descent

2. **Cell references**: How to handle A1, R1C1 notation?
   - Need Grid/Sheet abstraction
   - INDIRECT requires this
   - May need separate crate: `hyperformula-grid`

3. **Circular dependencies**: Detection and handling?
   - Requires dependency graph
   - Excel shows #REF! for circular refs
   - Implementation: topological sort, cycle detection

4. **Array formulas**: Spill behavior, dynamic arrays?
   - Modern Excel feature
   - Functions return arrays that "spill" into adjacent cells
   - FILTER, SORT, UNIQUE are array functions

5. **Internationalization**: Formula names in other languages?
   - Excel translates function names (SUM → SUMA in Spanish)
   - Consider: locale-aware registry
   - Low priority for now

6. **Arbitrary precision**: Should we support BigDecimal?
   - Excel uses 15-digit precision (IEEE 754 double)
   - Some financial calcs need more
   - Could add optional arbitrary precision mode

## Success Metrics

- **Coverage**: 400+ functions implemented
- **Compatibility**: 95%+ match with Excel on test suite
- **Performance**: 10x faster than JavaScript HyperFormula
- **Quality**: 100% test coverage, 0 clippy warnings
- **Documentation**: Every function documented with examples

## Timeline Summary

- **Week 1-2**: Phase 1 (Foundation expansion, 25 functions)
- **Week 3-6**: Phase 2 (High-priority, 100 functions total)
- **Week 7-10**: Phase 3 (Specialized, 250 functions total)
- **Week 11-12**: Phase 4 (Complete 400+, compatibility)

**Total**: ~12 weeks to full implementation

## Next Steps

1. Review and approve this plan
2. Set up project tracking (GitHub project board)
3. Begin Phase 1 implementation
4. Establish CI/CD pipeline with performance tracking
5. Create Excel test workbook for compatibility testing

---

**Last updated**: 2026-06-23  
**Status**: Initial draft  
**Owner**: AnzenDigital team
