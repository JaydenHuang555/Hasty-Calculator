use hastycalc::{eval, read::lexer, token::operator::executables};

#[test]
fn test() {
    let result = lexer::postfix("2 * (-10+3)2");
    for token in &result {
        println!("{}", token);
    }
    let end = eval::eval(&result);
    println!("{}", end);
    println!("{}", executables::EXPONENT.execute(2.0, 0.5));
}
