use hastycalc::{eval::{self, evaluate}, read::lexer, token::operator::executables};

#[test]
fn test() {
    match lexer::postfix("2.5*2") {
        Result::Ok(output) => {
            print!("Postfix tokens: ");
            for token in &output {
                print!("{} ", token);
            }
            println!();
            match evaluate::evaluate(&output) {
                Result::Ok(result) => println!("Result is: {}", result),
                Result::Err(err) => eprintln!("Found evaluation error: {}", err)
            }
        }
        Result::Err(err) => {
            eprintln!("Found lex error: {}", err);
        }
    }
}
