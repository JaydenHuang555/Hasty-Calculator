use hastycalc::{eval, read::lexer, token::operator::executables};

#[test]
fn test() {
    let result = lexer::postfix("2.5*2");
    for token in &result {
        println!("{}", token);
    }
    let end = eval::evaluate::evaluate(&result);
    println!("{}", end.unwrap());
}
