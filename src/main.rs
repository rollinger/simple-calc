use std::env;
mod parser;
mod calc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let expression: &str;

    // Check for arguments
    if args.len() < 2 {
        // TODO: Add user input
        eprintln!("Usage: {} <expression>", args[0]);
        //std::process::exit(1);
        expression = "120000/((4*5*8)*((3+3)*2))";
    } else {
        expression = &args[1];
    }

    match parser::sanity_check(expression) {
        false => {
            eprintln!("ERROR: Malformed Expression! Aborting...");
            return ();
        },
        true => {
            println!("Expression Good!");
        }
    }
    let result: f64 = calc::calculate_result(expression);
    println!("=> {} = {}", expression, result);
}
