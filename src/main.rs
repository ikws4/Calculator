mod parser;
mod calculator;

use std::io::{Read, Write};
use rustyline::{DefaultEditor, Result};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use crate::calculator::Calculator;

fn main() -> Result<()> {
    let mut repl = DefaultEditor::new()?;
    let mut calc = Calculator::new();

    loop {
        let line = repl.readline("> ");
        match line {
            Ok(expr) => {
                repl.add_history_entry(expr.as_str()).unwrap();

                match calc.eval(expr) {
                    Ok(value) => println!("{}", value),
                    Err(msg) => println!("{}", msg)
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

mod test {
    use crate::{Calculator};

    #[test]
    fn test_parse() {
        let mut calc = Calculator::new();

        let parse = |expr: &str| -> Result<f64, String> {
            calc.eval(expr.to_string())
        };

        assert_eq!(parse("1+2"), 3.);
        assert_eq!(parse("1+2*3"), 7.);
        assert_eq!(parse("(1+3)%3"), 1.);
    }
}
