use hastycalc::{eval, read::lexer};

#[test]
fn test() {
    let result = lexer::postfix("(-10+3)*2");
    for token in &result {
        println!("{}", token);
    }
    let end = eval::eval(&result);
    println!("{}", end);
}