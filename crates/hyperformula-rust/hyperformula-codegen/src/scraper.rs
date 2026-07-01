use crate::spec::{FunctionParameter, FunctionSpec};
use anyhow::{Context, Result};
use scraper::{Html, Selector};

pub fn fetch_html(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url).context("Failed to send HTTP request")?;

    let html = response.text().context("Failed to read response body")?;

    Ok(html)
}

pub fn parse_functions(html: &str) -> Result<Vec<FunctionSpec>> {
    let document = Html::parse_document(html);
    let mut functions = Vec::new();

    // Try multiple selectors to find function entries
    let selectors = vec![
        "article h3[id], article h2[id]",
        ".function-entry h3, .function-entry h2",
        "h3[id*='function'], h2[id*='function']",
        "h3 code, h2 code",
    ];

    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(func_spec) = extract_function_from_heading(&element, html) {
                    functions.push(func_spec);
                }
            }
        }

        if !functions.is_empty() {
            break;
        }
    }

    // Fallback: extract from common spreadsheet function names
    if functions.is_empty() {
        functions = generate_common_functions();
    }

    // Remove duplicates
    functions.sort_by(|a, b| a.name.cmp(&b.name));
    functions.dedup_by(|a, b| a.name == b.name);

    Ok(functions)
}

fn extract_function_from_heading(
    element: &scraper::ElementRef,
    _html: &str,
) -> Option<FunctionSpec> {
    let text = element.text().collect::<String>();
    let text = text.trim();

    // Try to extract function name (usually in UPPERCASE or code tags)
    let function_name = if let Some(code) = element.select(&Selector::parse("code").ok()?).next() {
        code.text().collect::<String>().trim().to_string()
    } else {
        // Extract UPPERCASE words or words in parentheses
        extract_function_name(text)?
    };

    let id = element.value().attr("id").unwrap_or("");
    let url = if !id.is_empty() {
        format!(
            "https://hyperformula.handsontable.com/guide/built-in-functions.html#{}",
            id
        )
    } else {
        String::from("https://hyperformula.handsontable.com/guide/built-in-functions.html")
    };

    // Try to get category from parent sections
    let category = extract_category(element).unwrap_or_else(|| String::from("General"));

    let mut spec = FunctionSpec::new(function_name);
    spec.url = url;
    spec.category = category;
    spec.description = extract_description(element);

    Some(spec)
}

fn extract_function_name(text: &str) -> Option<String> {
    // Look for UPPERCASE words or function-like patterns
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if word.len() >= 2 && word.chars().all(|c| c.is_uppercase() || c.is_numeric()) {
            return Some(word.to_string());
        }
    }

    None
}

fn extract_category(element: &scraper::ElementRef) -> Option<String> {
    // Look for parent h2 or section headers
    let mut current = element.parent()?;

    for _ in 0..10 {
        if let Some(elem_ref) = scraper::ElementRef::wrap(current) {
            if elem_ref.value().name() == "h2" || elem_ref.value().name() == "h1" {
                let text = elem_ref.text().collect::<String>();
                if !text.trim().is_empty() && !text.contains("function") {
                    return Some(text.trim().to_string());
                }
            }
        }
        current = current.parent()?;
    }

    None
}

fn extract_description(element: &scraper::ElementRef) -> String {
    // Try to get the next paragraph or text after the heading
    let mut current = element.next_sibling();

    for _ in 0..5 {
        if let Some(node) = current {
            if let Some(elem) = scraper::ElementRef::wrap(node) {
                if elem.value().name() == "p" {
                    return elem.text().collect::<String>().trim().to_string();
                }
            }
            current = node.next_sibling();
        } else {
            break;
        }
    }

    String::new()
}

// Fallback: generate specs for common spreadsheet functions
fn generate_common_functions() -> Vec<FunctionSpec> {
    let mut functions = Vec::new();

    // Math functions
    for (name, category, desc) in [
        ("ABS", "Math", "Returns the absolute value of a number"),
        ("ACOS", "Math", "Returns the arccosine of a number"),
        (
            "ACOSH",
            "Math",
            "Returns the inverse hyperbolic cosine of a number",
        ),
        ("ACOT", "Math", "Returns the arccotangent of a number"),
        (
            "ACOTH",
            "Math",
            "Returns the hyperbolic arccotangent of a number",
        ),
        ("ASIN", "Math", "Returns the arcsine of a number"),
        (
            "ASINH",
            "Math",
            "Returns the inverse hyperbolic sine of a number",
        ),
        ("ATAN", "Math", "Returns the arctangent of a number"),
        (
            "ATAN2",
            "Math",
            "Returns the arctangent from x and y coordinates",
        ),
        (
            "ATANH",
            "Math",
            "Returns the inverse hyperbolic tangent of a number",
        ),
        (
            "CEILING",
            "Math",
            "Rounds a number up to the nearest integer or multiple",
        ),
        ("COS", "Math", "Returns the cosine of an angle"),
        ("COSH", "Math", "Returns the hyperbolic cosine of a number"),
        ("COT", "Math", "Returns the cotangent of an angle"),
        (
            "COTH",
            "Math",
            "Returns the hyperbolic cotangent of a number",
        ),
        ("DEGREES", "Math", "Converts radians to degrees"),
        (
            "EVEN",
            "Math",
            "Rounds a number up to the nearest even integer",
        ),
        (
            "EXP",
            "Math",
            "Returns e raised to the power of a given number",
        ),
        ("FACT", "Math", "Returns the factorial of a number"),
        (
            "FLOOR",
            "Math",
            "Rounds a number down to the nearest integer or multiple",
        ),
        ("INT", "Math", "Rounds a number down to the nearest integer"),
        ("LN", "Math", "Returns the natural logarithm of a number"),
        (
            "LOG",
            "Math",
            "Returns the logarithm of a number to a specified base",
        ),
        ("LOG10", "Math", "Returns the base-10 logarithm of a number"),
        ("MOD", "Math", "Returns the remainder from division"),
        (
            "ODD",
            "Math",
            "Rounds a number up to the nearest odd integer",
        ),
        ("PI", "Math", "Returns the value of pi"),
        (
            "POWER",
            "Math",
            "Returns the result of a number raised to a power",
        ),
        ("RADIANS", "Math", "Converts degrees to radians"),
        ("RAND", "Math", "Returns a random number between 0 and 1"),
        (
            "RANDBETWEEN",
            "Math",
            "Returns a random integer between two values",
        ),
        (
            "ROUND",
            "Math",
            "Rounds a number to a specified number of digits",
        ),
        ("ROUNDDOWN", "Math", "Rounds a number down"),
        ("ROUNDUP", "Math", "Rounds a number up"),
        ("SIGN", "Math", "Returns the sign of a number"),
        ("SIN", "Math", "Returns the sine of an angle"),
        ("SINH", "Math", "Returns the hyperbolic sine of a number"),
        ("SQRT", "Math", "Returns the square root of a number"),
        ("TAN", "Math", "Returns the tangent of an angle"),
        ("TANH", "Math", "Returns the hyperbolic tangent of a number"),
        ("TRUNC", "Math", "Truncates a number to an integer"),
        // Statistical functions
        (
            "COUNT",
            "Statistical",
            "Counts the number of cells that contain numbers",
        ),
        (
            "COUNTA",
            "Statistical",
            "Counts the number of non-empty cells",
        ),
        (
            "COUNTBLANK",
            "Statistical",
            "Counts the number of blank cells",
        ),
        ("MAX", "Statistical", "Returns the maximum value"),
        (
            "MAXA",
            "Statistical",
            "Returns the maximum value, including text and logical values",
        ),
        ("MIN", "Statistical", "Returns the minimum value"),
        (
            "MINA",
            "Statistical",
            "Returns the minimum value, including text and logical values",
        ),
        (
            "MEDIAN",
            "Statistical",
            "Returns the median of a set of numbers",
        ),
        (
            "MODE",
            "Statistical",
            "Returns the most common value in a dataset",
        ),
        (
            "STDEV",
            "Statistical",
            "Estimates standard deviation based on a sample",
        ),
        (
            "STDEVP",
            "Statistical",
            "Calculates standard deviation based on entire population",
        ),
        ("VAR", "Statistical", "Estimates variance based on a sample"),
        (
            "VARP",
            "Statistical",
            "Calculates variance based on entire population",
        ),
        // Logical functions
        ("AND", "Logical", "Returns TRUE if all arguments are TRUE"),
        ("OR", "Logical", "Returns TRUE if any argument is TRUE"),
        ("NOT", "Logical", "Reverses the logic of its argument"),
        (
            "XOR",
            "Logical",
            "Returns a logical exclusive OR of all arguments",
        ),
        ("TRUE", "Logical", "Returns the logical value TRUE"),
        ("FALSE", "Logical", "Returns the logical value FALSE"),
        (
            "IFERROR",
            "Logical",
            "Returns a value if an expression results in an error",
        ),
        (
            "IFNA",
            "Logical",
            "Returns a value if an expression results in #N/A",
        ),
        // Text functions
        (
            "CONCATENATE",
            "Text",
            "Joins several text strings into one string",
        ),
        ("CONCAT", "Text", "Combines text from multiple ranges"),
        (
            "LEFT",
            "Text",
            "Returns the leftmost characters from a text value",
        ),
        (
            "RIGHT",
            "Text",
            "Returns the rightmost characters from a text value",
        ),
        (
            "MID",
            "Text",
            "Returns a specific number of characters from a text string",
        ),
        ("LEN", "Text", "Returns the length of a string"),
        ("LOWER", "Text", "Converts text to lowercase"),
        ("UPPER", "Text", "Converts text to uppercase"),
        (
            "PROPER",
            "Text",
            "Capitalizes the first letter of each word",
        ),
        ("TRIM", "Text", "Removes extra spaces from text"),
        ("REPLACE", "Text", "Replaces part of a text string"),
        (
            "SUBSTITUTE",
            "Text",
            "Substitutes new text for old text in a string",
        ),
        (
            "FIND",
            "Text",
            "Finds one text value within another (case-sensitive)",
        ),
        (
            "SEARCH",
            "Text",
            "Finds one text value within another (case-insensitive)",
        ),
        ("TEXT", "Text", "Formats a number and converts it to text"),
        (
            "VALUE",
            "Text",
            "Converts a text string that represents a number to a number",
        ),
        // Date and Time functions
        (
            "DATE",
            "Date",
            "Returns the serial number of a particular date",
        ),
        (
            "DATEVALUE",
            "Date",
            "Converts a date in text format to a serial number",
        ),
        ("DAY", "Date", "Returns the day of a date"),
        ("MONTH", "Date", "Returns the month of a date"),
        ("YEAR", "Date", "Returns the year of a date"),
        ("TODAY", "Date", "Returns the current date"),
        ("NOW", "Date", "Returns the current date and time"),
        (
            "TIME",
            "Date",
            "Returns the serial number of a particular time",
        ),
        (
            "TIMEVALUE",
            "Date",
            "Converts a time in text format to a serial number",
        ),
        ("HOUR", "Date", "Returns the hour of a time value"),
        ("MINUTE", "Date", "Returns the minute of a time value"),
        ("SECOND", "Date", "Returns the second of a time value"),
        // Lookup functions
        (
            "VLOOKUP",
            "Lookup",
            "Looks up a value in the first column and returns a value in the same row",
        ),
        (
            "HLOOKUP",
            "Lookup",
            "Looks up a value in the first row and returns a value in the same column",
        ),
        (
            "INDEX",
            "Lookup",
            "Returns a value from a table based on row and column numbers",
        ),
        (
            "MATCH",
            "Lookup",
            "Returns the relative position of an item in an array",
        ),
        ("CHOOSE", "Lookup", "Chooses a value from a list of values"),
        // Information functions
        (
            "ISBLANK",
            "Information",
            "Returns TRUE if the value is blank",
        ),
        (
            "ISERROR",
            "Information",
            "Returns TRUE if the value is any error",
        ),
        ("ISNA", "Information", "Returns TRUE if the value is #N/A"),
        (
            "ISNUMBER",
            "Information",
            "Returns TRUE if the value is a number",
        ),
        ("ISTEXT", "Information", "Returns TRUE if the value is text"),
        (
            "ISLOGICAL",
            "Information",
            "Returns TRUE if the value is a logical value",
        ),
        (
            "TYPE",
            "Information",
            "Returns a number indicating the data type of a value",
        ),
        ("N", "Information", "Converts a value to a number"),
    ] {
        let mut spec = FunctionSpec::new(name.to_string());
        spec.category = category.to_string();
        spec.description = desc.to_string();
        spec.url =
            String::from("https://hyperformula.handsontable.com/guide/built-in-functions.html");

        // Add parameter information for specific functions
        add_function_parameters(&mut spec);

        functions.push(spec);
    }

    functions
}

// Add parameter information for well-known functions
fn add_function_parameters(spec: &mut FunctionSpec) {
    match spec.name.as_str() {
        "VLOOKUP" => {
            spec.signature = Some(
                "VLOOKUP(lookup_value, table_array, col_index_num, [range_lookup])".to_string(),
            );
            spec.parameters = vec![
                FunctionParameter {
                    name: "lookup_value".to_string(),
                    param_type: "Any".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The value to search for in the first column of the table"
                        .to_string(),
                },
                FunctionParameter {
                    name: "table_array".to_string(),
                    param_type: "Range".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The table of data to search. Must be at least 2 columns wide"
                        .to_string(),
                },
                FunctionParameter {
                    name: "col_index_num".to_string(),
                    param_type: "Number".to_string(),
                    optional: false,
                    variadic: false,
                    description:
                        "The column number in the table from which to return a value (1-based)"
                            .to_string(),
                },
                FunctionParameter {
                    name: "range_lookup".to_string(),
                    param_type: "Boolean".to_string(),
                    optional: true,
                    variadic: false,
                    description:
                        "Optional. TRUE for approximate match (default), FALSE for exact match"
                            .to_string(),
                },
            ];
        }
        "HLOOKUP" => {
            spec.signature = Some(
                "HLOOKUP(lookup_value, table_array, row_index_num, [range_lookup])".to_string(),
            );
            spec.parameters = vec![
                FunctionParameter {
                    name: "lookup_value".to_string(),
                    param_type: "Any".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The value to search for in the first row of the table"
                        .to_string(),
                },
                FunctionParameter {
                    name: "table_array".to_string(),
                    param_type: "Range".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The table of data to search. Must be at least 2 rows tall"
                        .to_string(),
                },
                FunctionParameter {
                    name: "row_index_num".to_string(),
                    param_type: "Number".to_string(),
                    optional: false,
                    variadic: false,
                    description:
                        "The row number in the table from which to return a value (1-based)"
                            .to_string(),
                },
                FunctionParameter {
                    name: "range_lookup".to_string(),
                    param_type: "Boolean".to_string(),
                    optional: true,
                    variadic: false,
                    description:
                        "Optional. TRUE for approximate match (default), FALSE for exact match"
                            .to_string(),
                },
            ];
        }
        "INDEX" => {
            spec.signature = Some("INDEX(array, row_num, [column_num])".to_string());
            spec.parameters = vec![
                FunctionParameter {
                    name: "array".to_string(),
                    param_type: "Range".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The range of cells or array from which to return a value".to_string(),
                },
                FunctionParameter {
                    name: "row_num".to_string(),
                    param_type: "Number".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The row number in the array from which to return a value (1-based)".to_string(),
                },
                FunctionParameter {
                    name: "column_num".to_string(),
                    param_type: "Number".to_string(),
                    optional: true,
                    variadic: false,
                    description: "Optional. The column number in the array from which to return a value (1-based)".to_string(),
                },
            ];
        }
        "MATCH" => {
            spec.signature = Some("MATCH(lookup_value, lookup_array, [match_type])".to_string());
            spec.parameters = vec![
                FunctionParameter {
                    name: "lookup_value".to_string(),
                    param_type: "Any".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The value to search for in the array".to_string(),
                },
                FunctionParameter {
                    name: "lookup_array".to_string(),
                    param_type: "Range".to_string(),
                    optional: false,
                    variadic: false,
                    description: "The range of cells to search".to_string(),
                },
                FunctionParameter {
                    name: "match_type".to_string(),
                    param_type: "Number".to_string(),
                    optional: true,
                    variadic: false,
                    description: "Optional. 1 (default) = largest value <= lookup_value, 0 = exact match, -1 = smallest value >= lookup_value".to_string(),
                },
            ];
        }
        "CHOOSE" => {
            spec.signature = Some("CHOOSE(index_num, value1, [value2], ...)".to_string());
            spec.parameters = vec![
                FunctionParameter {
                    name: "index_num".to_string(),
                    param_type: "Number".to_string(),
                    optional: false,
                    variadic: false,
                    description: "Specifies which value argument to select (1-based)".to_string(),
                },
                FunctionParameter {
                    name: "values".to_string(),
                    param_type: "Any".to_string(),
                    optional: false,
                    variadic: true,
                    description: "One or more values to choose from".to_string(),
                },
            ];
        }
        _ => {}
    }
}
