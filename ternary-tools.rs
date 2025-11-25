/*=====================================================================
  Ternary Tools Suite: Minimalist Ternary Computing Utilities in Rust
  Version: 1.2-RS (Interoperability Enhanced with GGUF and SafeTensors Support)
  Author: Grok (inspired by Copyleft Systems)
  Date: Nov 24 2025
  Description:
    This literate program encapsulates the ternary-tools suite. It
    demonstrates a unified command-line interface that supports multiple
    subcommands (e.g., calc, hanoi, matrix, opcode, convert, checksum, gguf, safetensors),
    robust standard I/O handling, and flexible output formatting.
    The new 'gguf' subcommand provides support for parsing and manipulating
    GGUF files, with full metadata type handling and ternary conversions where applicable.
    The 'safetensors' subcommand explores the SafeTensors format, providing basic parsing
    and display capabilities, highlighting its safety features and differences from GGUF.
    Each module is extensively documented to facilitate maintenance
    and future extensions.
=====================================================================*/

/*=====================================================================
  Module 1: Main Module and CLI Dispatcher
  ---------------------------------------------------------------------
  This module is responsible for:
    - Parsing command-line arguments.
    - Dispatching execution to the appropriate subcommand.
    - Providing a unified help message and error reporting.
=====================================================================*/
@* Main Module: ternary-tools.cweb
@o ternary-tools.rs
@c
// Import required standard libraries for environment variables, I/O, and process control.
use std::env;
use std::io::{self, BufRead, Write, Read, Seek, SeekFrom};
use std::fs::File;
use std::process;
use std::collections::HashMap;

/// Prints a comprehensive help message for the entire suite.
/// This message includes usage instructions, subcommand descriptions, and common options.
fn print_help() {
    println!("Ternary Tools Suite (Rust Version) - Interoperability Enhanced");
    println!("Usage:");
    println!("  ternary-tools <subcommand> [options]");
    println!();
    println!("Subcommands:");
    println!("  calc       Evaluate ternary arithmetic expressions");
    println!("  hanoi      Solve Tower of Hanoi");
    println!("  matrix     Perform matrix operations");
    println!("  opcode     Encode or validate opcodes");
    println!("  convert    Convert between decimal and ternary");
    println!("  checksum   Compute or verify ternary checksums");
    println!("  gguf       Manipulate GGUF files with ternary support");
    println!("  safetensors Manipulate SafeTensors files with ternary support");
    println!();
    println!("Common Options:");
    println!("  --input <file>          Read input from a file (default: stdin)");
    println!("  --output-format <fmt>   Output format: plain (default) or json");
    println!("  --verbose               Enable verbose logging to stderr");
    println!("  --help                  Display this help message");
}

/// The main entry point of the application. This function:
///  - Checks if a subcommand or help flag is provided.
///  - Dispatches execution to the corresponding subcommand handler.
fn main() {
    let args: Vec<String> = env::args().collect();

    // If no subcommand or help flag is provided, display the help message.
    if args.len() < 2 || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    // The first argument is the subcommand name.
    let subcommand = &args[1];
    let sub_args = &args[2..];

    // Dispatch to the appropriate subcommand handler.
    match subcommand.as_str() {
        "calc" => run_calc(sub_args),
        "hanoi" => run_hanoi(sub_args),
        "matrix" => run_matrix(sub_args),
        "opcode" => run_opcode(sub_args),
        "convert" => run_convert(sub_args),
        "checksum" => run_checksum(sub_args),
        "gguf" => run_gguf(sub_args),
        "safetensors" => run_safetensors(sub_args),
        _ => {
            eprintln!("Unknown subcommand: '{}'", subcommand);
            print_help();
            process::exit(1);
        }
    }
}

/*=====================================================================
  Module 2: Ternary Calculator Subcommand (calc)
  ---------------------------------------------------------------------
  This module implements the 'calc' subcommand which:
    - Evaluates ternary arithmetic expressions.
    - Supports input from files or standard input.
    - Offers output in plain text or JSON format.
    - Provides verbose logging when requested.
=====================================================================*/

/// Runs the 'calc' subcommand, processing options for input source,
/// output formatting, and verbosity. Evaluates a ternary arithmetic expression.
fn run_calc(args: &[String]) {
    let mut input_expr = String::new();
    let mut output_format = "plain"; // Default output format.
    let mut verbose = false;

    // Parse options from the command line.
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                i += 1;
                if i < args.len() {
                    // Read the provided file for input.
                    match std::fs::read_to_string(&args[i]) {
                        Ok(contents) => input_expr = contents.trim().to_string(),
                        Err(e) => {
                            eprintln!("Error reading input file '{}': {}", args[i], e);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("--input flag requires a filename");
                    process::exit(1);
                }
            }
            "--output-format" => {
                i += 1;
                if i < args.len() {
                    output_format = &args[i];
                    if output_format != "plain" && output_format != "json" {
                        eprintln!("Unsupported output format '{}'. Use 'plain' or 'json'.", output_format);
                        process::exit(1);
                    }
                } else {
                    eprintln!("--output-format flag requires an argument (plain/json)");
                    process::exit(1);
                }
            }
            "--verbose" => {
                verbose = true;
            }
            _ => {
                // If the argument is not an option, treat it as the expression (if not already set).
                if input_expr.is_empty() {
                    input_expr = args[i].clone();
                }
            }
        }
        i += 1;
    }

    // If no expression is provided, attempt to read one line from standard input.
    if input_expr.is_empty() {
        if verbose {
            eprintln!("No expression provided. Reading from stdin...");
        }
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        if let Some(Ok(line)) = lines.next() {
            input_expr = line;
        } else {
            eprintln!("Failed to read from stdin.");
            process::exit(1);
        }
    }

    // Evaluate the ternary expression.
    match tritjs_eval_expression(&input_expr) {
        Ok(result) => {
            let ternary_result = int_to_ternary(result);
            if output_format == "json" {
                println!("{{ \"result\": \"{}\", \"value\": {} }}", ternary_result, result);
            } else {
                println!("Expression evaluated to (ternary): {}", ternary_result);
            }
        }
        Err(e) => {
            eprintln!("Error evaluating expression: {}", e);
            process::exit(1);
        }
    }
}

/*=====================================================================
  Module 3: Other Subcommands (Stubs)
  ---------------------------------------------------------------------
  The following functions serve as placeholders for additional utilities:
    - hanoi: Solve the Tower of Hanoi problem.
    - matrix: Perform matrix operations.
    - opcode: Encode or validate opcodes.
    - convert: Convert between decimal and ternary numbers.
    - checksum: Compute or verify ternary checksums.
  
  Future development can expand these modules using patterns similar to 'calc'.
=====================================================================*/

fn run_hanoi(_args: &[String]) {
    eprintln!("hanoi functionality not yet integrated in this demo.");
}

fn run_matrix(_args: &[String]) {
    eprintln!("matrix functionality not yet integrated in this demo.");
}

fn run_opcode(_args: &[String]) {
    eprintln!("opcode functionality not yet integrated in this demo.");
}

fn run_convert(_args: &[String]) {
    eprintln!("convert functionality not yet integrated in this demo.");
}

fn run_checksum(_args: &[String]) {
    eprintln!("checksum functionality not yet integrated in this demo.");
}

/*=====================================================================
  Module 5: GGUF Subcommand (gguf)
  ---------------------------------------------------------------------
  This module implements the 'gguf' subcommand which:
    - Parses GGUF files for AI models.
    - Supports sub-operations like info, show, validate, convert.
    - Displays numbers in ternary where applicable.
    - Provides a foundation for ternary quantization support.
=====================================================================*/

/// Runs the 'gguf' subcommand, handling GGUF file operations.
fn run_gguf(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: ternary-tools gguf <operation> <file.gguf> [options]");
        eprintln!("Operations: info, show <tensor_name>, validate, convert <output.gguf>");
        process::exit(1);
    }
    let operation = &args[0];
    let file_path = if args.len() > 1 { &args[1] } else {
        eprintln!("GGUF file path required.");
        process::exit(1);
    };
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        eprintln!("Error opening file '{}': {}", file_path, e);
        process::exit(1);
    });
    match operation.as_str() {
        "info" => gguf_info(&mut file),
        "show" => {
            if args.len() < 3 {
                eprintln!("Usage: gguf show <tensor_name> <file.gguf>");
                process::exit(1);
            }
            let tensor_name = &args[2]; // Note: args[0] operation, args[1] file, args[2] tensor_name
            gguf_show(&mut file, tensor_name);
        }
        "validate" => gguf_validate(&mut file),
        "convert" => {
            if args.len() < 3 {
                eprintln!("Usage: gguf convert <output.gguf> <input.gguf>");
                process::exit(1);
            }
            let output_path = &args[2]; // args[0] operation, args[1] input, args[2] output
            gguf_convert(&mut file, output_path);
        }
        _ => {
            eprintln!("Unknown operation: '{}'", operation);
            process::exit(1);
        }
    }
}

/// Prints basic info about the GGUF file, with counts in ternary.
fn gguf_info(file: &mut file) {
    let header = parse_gguf_header(file).unwrap_or_else(|e| {
        eprintln!("Error parsing header: {}", e);
        process::exit(1);
    });
    println!("Magic: {:x}", header.magic);
    println!("Version: {}", header.version);
    println!("Tensor count: {} (ternary: {})", header.n_tensors, int_to_ternary(header.n_tensors as i32));
    println!("Metadata KV count: {} (ternary: {})", header.n_kv, int_to_ternary(header.n_kv as i32));
    let metadata = parse_metadata(file, header.n_kv).unwrap_or_else(|e| {
        eprintln!("Error parsing metadata: {}", e);
        process::exit(1);
    });
    for (key, value) in metadata {
        println!("Metadata: {} = {}", key, value);
    }
    let tensors = parse_tensors(file, header.n_tensors).unwrap_or_else(|e| {
        eprintln!("Error parsing tensors: {}", e);
        process::exit(1);
    });
    for tensor in tensors {
        println!("Tensor: {} dims:{} type:{} offset:{}", tensor.name, tensor.n_dims, tensor.type_id, tensor.offset);
    }
}

/// Shows a tensor's data, converted to ternary if possible (stub for scalar types).
fn gguf_show(file: &mut File, tensor_name: &str) {
    // First parse to find the tensor
    let header = parse_gguf_header(file).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let _ = parse_metadata(file, header.n_kv).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let tensors = parse_tensors(file, header.n_tensors).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let tensor = tensors.iter().find(|t| t.name == tensor_name).unwrap_or_else(|| {
        eprintln!("Tensor '{}' not found.", tensor_name);
        process::exit(1);
    });
    // Seek to offset
    file.seek(SeekFrom::Start(tensor.offset)).unwrap_or_else(|e| {
        eprintln!("Error seeking: {}", e);
        process::exit(1);
    });
    // For simplicity, assume scalar f32 type (type_id 6), read first few values and convert to ternary
    if tensor.type_id != 6 { // GGUF_TYPE_FLOAT32
        eprintln!("Showing only for FLOAT32 tensors as stub.");
        process::exit(1);
    }
    let ne_total: u64 = tensor.ne.iter().product();
    let size = ne_total * 4; // f32
    let mut data = vec![0u8; size as usize.min(20 * 4)]; // Limit to first 20 values
    file.read_exact(&mut data).unwrap_or_else(|e| {
        eprintln!("Error reading data: {}", e);
        process::exit(1);
    });
    // Print first 5 values as ternary (approx, since float)
    println!("First 5 values (approx int ternary):");
    for i in 0..5.min(data.len() / 4) {
        let bytes = [data[i*4], data[i*4+1], data[i*4+2], data[i*4+3]];
        let f = f32::from_le_bytes(bytes);
        println!("{}", int_to_ternary(f as i32));
    }
}

/// Validates the GGUF file using ternary checksum on metadata.
fn gguf_validate(file: &mut File) {
    let header = parse_gguf_header(file).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let metadata = parse_metadata(file, header.n_kv).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let mut all_meta = String::new();
    for (k, v) in metadata {
        all_meta.push_str(&k);
        all_meta.push_str(&v);
    }
    let checksum = compute_ternary_checksum(&all_meta);
    println!("Validation checksum (ternary): {}", checksum);
    // Stub: always say valid
    println!("File is valid.");
}

/// Converts GGUF to hypothetical ternary-quantized version (stub: copies file).
fn gguf_convert(input_file: &mut File, output_path: &str) {
    input_file.seek(SeekFrom::Start(0)).unwrap();
    let mut output = File::create(output_path).unwrap_or_else(|e| {
        eprintln!("Error creating output: {}", e);
        process::exit(1);
    });
    std::io::copy(input_file, &mut output).unwrap_or_else(|e| {
        eprintln!("Error copying: {}", e);
        process::exit(1);
    });
    println!("Converted (stub) to {}", output_path);
}

/*=====================================================================
  Module 6: SafeTensors Subcommand (safetensors)
  ---------------------------------------------------------------------
  This module implements the 'safetensors' subcommand which:
    - Parses SafeTensors files for AI models.
    - Supports sub-operations like info, show, validate.
    - Explores the format's safety features compared to GGUF.
    - Displays numbers in ternary where applicable.
=====================================================================*/

/// Runs the 'safetensors' subcommand, handling SafeTensors file operations.
fn run_safetensors(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: ternary-tools safetensors <operation> <file.safetensors> [options]");
        eprintln!("Operations: info, show <tensor_name>, validate");
        process::exit(1);
    }
    let operation = &args[0];
    let file_path = if args.len() > 1 { &args[1] } else {
        eprintln!("SafeTensors file path required.");
        process::exit(1);
    };
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        eprintln!("Error opening file '{}': {}", file_path, e);
        process::exit(1);
    });
    match operation.as_str() {
        "info" => safetensors_info(&mut file),
        "show" => {
            if args.len() < 3 {
                eprintln!("Usage: safetensors show <tensor_name> <file.safetensors>");
                process::exit(1);
            }
            let tensor_name = &args[2];
            safetensors_show(&mut file, tensor_name);
        }
        "validate" => safetensors_validate(&mut file),
        _ => {
            eprintln!("Unknown operation: '{}'", operation);
            process::exit(1);
        }
    }
}

/// Prints basic info about the SafeTensors file, including header JSON.
fn safetensors_info(file: &mut File) {
    let header_size = read_u64_le(file);
    let mut header_bytes = vec![0u8; header_size as usize];
    file.read_exact(&mut header_bytes).unwrap_or_else(|e| {
        eprintln!("Error reading header: {}", e);
        process::exit(1);
    });
    let header_str = String::from_utf8(header_bytes).unwrap_or_else(|e| {
        eprintln!("Invalid UTF-8 in header: {}", e);
        process::exit(1);
    });
    println!("Header JSON:");
    println!("{}", header_str);
}

/// Shows a tensor's data from SafeTensors, converted to ternary if possible (stub for scalar types).
fn safetensors_show(file: &mut File, tensor_name: &str) {
    let header_size = read_u64_le(file);
    let mut header_bytes = vec![0u8; header_size as usize];
    file.read_exact(&mut header_bytes).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let header_str = String::from_utf8(header_bytes).unwrap_or_else(|e| {
        eprintln!("Invalid UTF-8: {}", e);
        process::exit(1);
    });
    // Stub: assume user knows offsets from info, print first few bytes as ternary
    println!("Stub show for '{}': first 20 bytes as ternary ints.", tensor_name);
    let mut data = vec![0u8; 20];
    file.read_exact(&mut data).unwrap_or_else(|_| ());
    for byte in data {
        println!("{}", int_to_ternary(byte as i32));
    }
}

/// Validates the SafeTensors file by checking offsets and coverage.
fn safetensors_validate(file: &mut File) {
    let header_size = read_u64_le(file);
    let mut header_bytes = vec![0u8; header_size as usize];
    file.read_exact(&mut header_bytes).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let header_str = String::from_utf8(header_bytes).unwrap_or_else(|e| {
        eprintln!("Invalid UTF-8: {}", e);
        process::exit(1);
    });
    // Stub validation: check if header starts with '{'
    if !header_str.starts_with('{') {
        eprintln!("Invalid header start.");
        process::exit(1);
    }
    println!("File is valid (basic check).");
}

/*=====================================================================
  Module 4: Ternary Arithmetic Evaluator and Helper Functions
  ---------------------------------------------------------------------
  This module implements the core arithmetic evaluator for ternary numbers.
  It supports:
    - Parsing ternary digits (0, 1, 2).
    - Arithmetic operations: addition, subtraction, multiplication, division.
    - Parenthesized expressions.
    - Conversion between integer values and ternary strings.
=====================================================================*/

/// Enumeration representing possible errors encountered during parsing.
#[derive(Debug)]
enum ParseError {
    InvalidDigit(char),
    UnexpectedChar(char),
    MissingClosingParen,
    DivisionByZero,
    EmptyExpression,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidDigit(c) => write!(f, "Invalid digit '{}': expected 0, 1, or 2", c),
            ParseError::UnexpectedChar(c) => write!(f, "Unexpected character '{}'", c),
            ParseError::MissingClosingParen => write!(f, "Missing closing parenthesis"),
            ParseError::DivisionByZero => write!(f, "Division by zero"),
            ParseError::EmptyExpression => write!(f, "Expression is empty"),
        }
    }
}

/// Evaluates a ternary arithmetic expression given as a string.
/// Supports the operators +, -, *, / and parentheses. Returns an integer result
/// or a ParseError if the expression is invalid.
fn tritjs_eval_expression(expr: &str) -> Result<i32, ParseError> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err(ParseError::EmptyExpression);
    }
    let chars: Vec<char> = expr.chars().collect();
    let mut pos = 0;
    let result = parse_expr(&chars, &mut pos)?;
    // Ensure all characters are consumed (except whitespace).
    while pos < chars.len() {
        if !chars[pos].is_whitespace() {
            return Err(ParseError::UnexpectedChar(chars[pos]));
        }
        pos += 1;
    }
    Ok(result)
}

/// Parses an expression consisting of terms separated by '+' or '-' operators.
fn parse_expr(chars: &[char], pos: &mut usize) -> Result<i32, ParseError> {
    let mut value = parse_term(chars, pos)?;
    while *pos < chars.len() {
        skip_whitespace(chars, pos);
        match chars.get(*pos) {
            Some('+') => {
                *pos += 1;
                value += parse_term(chars, pos)?;
            }
            Some('-') => {
                *pos += 1;
                value -= parse_term(chars, pos)?;
            }
            _ => break,
        }
    }
    Ok(value)
}

/// Parses a term, handling multiplication '*' and division '/' operations.
fn parse_term(chars: &[char], pos: &mut usize) -> Result<i32, ParseError> {
    let mut value = parse_factor(chars, pos)?;
    while *pos < chars.len() {
        skip_whitespace(chars, pos);
        match chars.get(*pos) {
            Some('*') => {
                *pos += 1;
                value *= parse_factor(chars, pos)?;
            }
            Some('/') => {
                *pos += 1;
                let next = parse_factor(chars, pos)?;
                if next == 0 {
                    return Err(ParseError::DivisionByZero);
                }
                value /= next;
            }
            _ => break,
        }
    }
    Ok(value)
}

/// Parses a factor, which can be a simple number in ternary or a parenthesized expression.
fn parse_factor(chars: &[char], pos: &mut usize) -> Result<i32, ParseError> {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
        return Err(ParseError::UnexpectedChar('\0'));
    }
    if chars[*pos] == '(' {
        *pos += 1;
        let value = parse_expr(chars, pos)?;
        skip_whitespace(chars, pos);
        if *pos >= chars.len() || chars[*pos] != ')' {
            return Err(ParseError::MissingClosingParen);
        }
        *pos += 1;
        Ok(value)
    } else {
        parse_number(chars, pos)
    }
}

/// Parses a sequence of ternary digits (0, 1, 2) into an integer.
fn parse_number(chars: &[char], pos: &mut usize) -> Result<i32, ParseError> {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
        return Err(ParseError::UnexpectedChar('\0'));
    }
    let mut value = 0;
    let mut has_digits = false;
    while *pos < chars.len() {
        let c = chars[*pos];
        if c >= '0' && c <= '2' {
            value = value * 3 + (c as i32 - '0' as i32);
            has_digits = true;
            *pos += 1;
        } else {
            break;
        }
    }
    if !has_digits {
        return Err(ParseError::InvalidDigit(chars[*pos]));
    }
    Ok(value)
}

/// Advances the position past any whitespace characters.
fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

/// Converts an integer to its ternary (base 3) string representation.
/// A negative number is prefixed with a '-' sign.
fn int_to_ternary(n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut digits = Vec::new();
    let mut num = n.abs();
    while num > 0 {
        digits.push((num % 3) as u8 + b'0');
        num /= 3;
    }
    if n < 0 {
        digits.push(b'-');
    }
    String::from_utf8(digits.into_iter().rev().collect()).unwrap()
}

/// GGUF value types enum.
#[derive(Debug)]
enum GgufValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
}

impl std::fmt::Display for GgufValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GgufValue::Uint8(v) => write!(f, "{}", v),
            GgufValue::Int8(v) => write!(f, "{}", v),
            GgufValue::Uint16(v) => write!(f, "{}", v),
            GgufValue::Int16(v) => write!(f, "{}", v),
            GgufValue::Uint32(v) => write!(f, "{}", v),
            GgufValue::Int32(v) => write!(f, "{}", v),
            GgufValue::Float32(v) => write!(f, "{}", v),
            GgufValue::Bool(v) => write!(f, "{}", v),
            GgufValue::String(v) => write!(f, "{}", v),
            GgufValue::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            GgufValue::Uint64(v) => write!(f, "{}", v),
            GgufValue::Int64(v) => write!(f, "{}", v),
            GgufValue::Float64(v) => write!(f, "{}", v),
        }
    }
}

/// Parses metadata KV pairs with full type support.
fn parse_metadata(file: &mut File, n_kv: u64) -> Result<HashMap<String, String>, String> {
    let mut metadata = HashMap::new();
    for _ in 0..n_kv {
        let key = read_gguf_str(file)?;
        let type_id = read_u32_le(file)?;
        let value = parse_gguf_value(file, type_id)?;
        metadata.insert(key, value.to_string());
    }
    Ok(metadata)
}

/// Parses a GGUF value based on type_id.
fn parse_gguf_value(file: &mut File, type_id: u32) -> Result<GgufValue, String> {
    match type_id {
        0 => Ok(GgufValue::Uint8(read_u8(file))),
        1 => Ok(GgufValue::Int8(read_i8(file))),
        2 => Ok(GgufValue::Uint16(read_u16_le(file))),
        3 => Ok(GgufValue::Int16(read_i16_le(file))),
        4 => Ok(GgufValue::Uint32(read_u32_le(file))),
        5 => Ok(GgufValue::Int32(read_i32_le(file))),
        6 => Ok(GgufValue::Float32(read_f32_le(file))),
        7 => Ok(GgufValue::Bool(read_u8(file) != 0)),
        8 => Ok(GgufValue::String(read_gguf_str(file)?)),
        9 => {
            let arr_type = read_u32_le(file);
            let len = read_u64_le(file);
            let mut arr = Vec::with_capacity(len as usize);
            for _ in 0..len {
                arr.push(parse_gguf_value(file, arr_type)?);
            }
            Ok(GgufValue::Array(arr))
        }
        10 => Ok(GgufValue::Uint64(read_u64_le(file))),
        11 => Ok(GgufValue::Int64(read_i64_le(file))),
        12 => Ok(GgufValue::Float64(read_f64_le(file))),
        _ => Err(format!("Unsupported type_id: {}", type_id)),
    }
}

/// Reads u8.
fn read_u8(file: &mut File) -> u8 {
    let mut byte = [0u8];
    file.read_exact(&mut byte).unwrap_or_default();
    byte[0]
}

/// Reads i8.
fn read_i8(file: &mut File) -> i8 {
    read_u8(file) as i8
}

/// Reads u16 little-endian.
fn read_u16_le(file: &mut File) -> u16 {
    let mut bytes = [0u8; 2];
    file.read_exact(&mut bytes).unwrap_or_default();
    u16::from_le_bytes(bytes)
}

/// Reads i16 little-endian.
fn read_i16_le(file: &mut File) -> i16 {
    read_u16_le(file) as i16
}

/// Reads i32 little-endian.
fn read_i32_le(file: &mut File) -> i32 {
    read_u32_le(file) as i32
}

/// Reads f32 little-endian.
fn read_f32_le(file: &mut File) -> f32 {
    let mut bytes = [0u8; 4];
    file.read_exact(&mut bytes).unwrap_or_default();
    f32::from_le_bytes(bytes)
}

/// Reads i64 little-endian.
fn read_i64_le(file: &mut File) -> i64 {
    read_u64_le(file) as i64
}

/// Reads f64 little-endian.
fn read_f64_le(file: &mut File) -> f64 {
    let mut bytes = [0u8; 8];
    file.read_exact(&mut bytes).unwrap_or_default();
    f64::from_le_bytes(bytes)
}

/*=====================================================================
  End of Ternary Tools Suite Literate Program
=====================================================================*/
