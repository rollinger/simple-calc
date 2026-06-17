pub fn calculate_result(expr: &str) -> f64 {
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
    let _e_len: usize = expr.len();
    0.0
}