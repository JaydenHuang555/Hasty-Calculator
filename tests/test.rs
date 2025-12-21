use hastycalc::read::lexer;


#[test]
fn test() {
    let result = lexer::postfix("1+2*3");
    for token in result {
        println!("{}", token);
    }
}