use hastycalc::{eval, read::lexer};

#[test]
fn test() {
    let result = lexer::postfix("(10+20)*30");
    for token in &result {
        println!("{}", token);
    }
    let end = eval::eval(&result);
    println!("{}", end);
}