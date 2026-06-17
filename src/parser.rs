const ALLOWED_CHARS: &str = "1234567890.+-/*() ";
const ALLOWED_NUMBERS: &str = "1234567890"; 
const ALLOWED_OPS: &str = "+-/*";
const FLOATING_POINT: char = '.';
const BRACKET_OPEN: char = '(';
const BRACKET_CLOSE: char = ')';

/// One-Pass sanity check for expressions
/// Returns bool weather the expression is correctly formed
pub fn sanity_check(expression: &str) -> bool {
    let mut nesting: i16 = 0;
    let mut last: Option<char> = None;
    for current in expression.chars() {
        if !ALLOWED_CHARS.contains(current) { return false; }
		nesting += match current {
			BRACKET_OPEN => 1,
			BRACKET_CLOSE => -1,
			_ => 0,
		};
		if last.is_some() {
			let last = last.unwrap();
			// Disallow floating point without being preceeded or followed by a number
			if current == FLOATING_POINT && !ALLOWED_NUMBERS.contains(last) { return false }
			if last == FLOATING_POINT && !ALLOWED_NUMBERS.contains(current) { return false }
			// Disallow double operation codes
			if ALLOWED_OPS.contains(last) && ALLOWED_OPS.contains(current) { return false }
			// Disallow empty brackets
			if last == BRACKET_OPEN && current == BRACKET_CLOSE { return false }
		}
		// Continue with the current as the last.
		last = Some(current);
    }
	// Disallow skewed nesting
    if nesting != 0 { return false; }
	// passed sanity check
    return true;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_pass_sanity_check() {
		let succeeding_expressions: [&str; 5] = [
			"(1+2)", "1-2+3", "12/3", "11/(23-4*6)", "3.0*12"
		];
		for expr in succeeding_expressions {
			assert_eq!(sanity_check(expr), true);
		}
	}

	#[test]
	fn should_fail_sanity_check() {
		let failing_expressions: [&str; 6] = [
			"1+(2", "(1)+2)", "1+.2", "(1+2.)", "1//2", "1+2()-3"
		];
		for expr in failing_expressions {
			assert_eq!(sanity_check(expr), false);
		}
		
	}
}