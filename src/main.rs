use lambda_calculator::Interpreter;
use clap::App;
use std::io;
use std::io::Write;
use std::convert::identity;

fn main() {
    let app = App::new("Lambda-calculator")
        .version("alpha")
        .author("Dmytro Polunin")
        .about("Tool for working with untyped lambda calculus");
    let it = Interpreter::new();
    repl(">> ", |s| it.run_code(&s).unwrap_or_else(identity));
}

fn repl(s: &str, mut f: impl FnMut(String) -> String) {
    loop {
        print!("{}", s);
        io::stdout().flush().unwrap();
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("Error when read line");
        data.pop();
        match data.as_str() {
            "exit" => return,
            _ => println!("{}", f(data)),
        }
    }
}
