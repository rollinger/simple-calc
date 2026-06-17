#[allow(dead_code)]
use std::io;



fn get_input() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Math statement required");
    return s.trim().to_string();
}

fn get_result(s: &String) -> f64 {
    let mut _parsed_s:[f64; 5];
    let mut result: f64 = 0.0;
    // parse
    // calc
    return result;
}

fn calculate_result(expr: &str) -> f64 {
    /* return the result of the expression
    - sanity check: allowed & matching brackets
    - extract ops: sequence of recusive math ops 
    Example: "(120000/((4*5*8)*((3+3)*2)))":
        r = rec("120000/((4*5*8)*((3+3)*2))")       => 5) 120000 / 1920 = 62.5
                rec("(4*5*8)*((3+3)*2)")            => 4) 160 * 12 = 1920
                    rec("(3+3)*2")                  => 2) 12
                        rec("3+3")                  => 1) 6
                    rec("4*5*8")                    => 3) 160
     */
    0.0
}

const ALLOWED_CHARS: &str = "1234567890.+-/*() ";
const ALLOWED_NUMBERS: &str = "1234567890."; 
const ALLOWED_OPS: &str = "+-/*";
const BRACKET_OPEN: char = '(';
const BRACKET_CLOSE: char = ')';


fn sanity_check(expression: &str) -> Result<bool, &str> {
    let mut n_brackets: i16 = 0;
    let mut operator = false;
    for c in expression.chars() {
        if !ALLOWED_CHARS.contains(c) {
            return Err("Malformed Expression: Unknown characters");
        }
        if c == BRACKET_OPEN {
            n_brackets += 1;
        } else if c == BRACKET_CLOSE {
            n_brackets -= 1;
        }
        if ALLOWED_OPS.contains(c) {
            operator = true
        } else {
            operator = false
        }
    }
    if n_brackets != 0 {
        return Err("Malformed Expression: Brackets mismatch");
    }
    if operator == true {
        return Err("Malformed Expression: Dangling operator");
    }
    return Ok(true);
}

fn main() {
    let expression_string: &str = "120000/((4*5*8)*((3+3)*2))";
    let result: f64 = calculate_result(expression_string);
    assert_eq!(result, 62.5);
    println!(">:_ {} => {}", expression_string, result);
}
