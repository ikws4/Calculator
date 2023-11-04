use std::collections::HashMap;
use crate::calculator::Function::*;
use crate::parser::Parser;

pub(crate) enum Function {
    OneArg(fn(f64) -> f64),
    TwoArg(fn(f64, f64) -> f64),
    ThreeArg(fn(f64, f64, f64) -> f64),
}

/// Grammar
///   expression: addition
///   addition: multiplication ('+' | '-' multiplication)*
///   multiplication: unary ('*' | '/' | '%' | '^' unary)*
///   unary: '-'? parentheses
///   parentheses: '(' expression ')' | atom
///   atom: number | call
///   number: [0-9]+ ('.' [0-9]+)?
///   call: identifier ('(' arguments ')')?
///   identifier: [a-zA-Z][a-zA-Z0-9]*
///   arguments: expression (',' expression)*
pub struct Calculator {
    parser: Parser,
    functions: HashMap<String, Function>,
    constants: HashMap<String, f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            parser: Parser::new("".to_string()),
            functions: HashMap::from([
                // @formatter:off
                ("abs".to_string(), OneArg(|a| a.abs())),
                ("ceil".to_string(), OneArg(|a| a.ceil())),
                ("floor".to_string(), OneArg(|a| a.floor())),
                ("round".to_string(), OneArg(|a| a.round())),
                ("sign".to_string(), OneArg(|a| a.signum())),

                ("sin".to_string(), OneArg(|a| a.sin())),
                ("cos".to_string(), OneArg(|a| a.cos())),
                ("tan".to_string(), OneArg(|a| a.tan())),
                ("asin".to_string(), OneArg(|a| a.asin())),
                ("acos".to_string(), OneArg(|a| a.acos())),
                ("atan".to_string(), OneArg(|a| a.atan())),

                ("ln".to_string(), OneArg(|a| a.ln())),
                ("log".to_string(), TwoArg(|a,b| b.log(a))),
                ("sqrt".to_string(), OneArg(|a| a.sqrt())),

                ("max".to_string(), TwoArg(|a, b| a.max(b))),
                ("min".to_string(), TwoArg(|a, b| a.min(b))),

                ("clamp".to_string(), ThreeArg(|a, b, c| a.clamp(b, c))),
                ("clamp01".to_string(), OneArg(|a| a.clamp(0., 1.))),
                // @formatter:on
            ]),
            constants: HashMap::from([
                ("pi".to_string(), std::f64::consts::PI),
                ("e".to_string(), std::f64::consts::E),
            ])
        }
    }

    pub fn eval(&mut self, expr: String) -> Result<f64, String> {
        self.parser = Parser::new(expr);
        self.expression()
    }

    fn expression(&mut self) -> Result<f64, String> {
        self.addition()
    }

    fn addition(&mut self) -> Result<f64, String> {
        let mut ret = self.multiplication()?;

        while let token = self.parser.peek() {
            match token {
                '+' => {
                    self.parser.advance();
                    ret += self.multiplication()?;
                }
                '-' => {
                    self.parser.advance();
                    ret -= self.multiplication()?;
                }
                _ => break
            }
        }

        Ok(ret)
    }

    fn multiplication(&mut self) -> Result<f64, String> {
        let mut ret = self.unary()?;

        while let token = self.parser.peek() {
            match token {
                '*' => {
                    self.parser.advance();
                    ret *= self.unary()?;
                }
                '/' => {
                    self.parser.advance();
                    ret /= self.unary()?;
                }
                '%' => {
                    self.parser.advance();
                    ret %= self.unary()?;
                }
                '^' => {
                    self.parser.advance();
                    ret = ret.powf(self.unary()?);
                }
                _ => break
            }
        }

        Ok(ret)
    }

    fn unary(&mut self) -> Result<f64, String> {
        match self.parser.peek() {
            '-' => {
                self.parser.advance();
                Ok(-self.parentheses()?)
            }
            _ => self.parentheses()
        }
    }

    fn parentheses(&mut self) -> Result<f64, String> {
        match self.parser.peek() {
            '(' => {
                self.parser.advance();
                let ret = self.expression();
                self.parser.consume(')', "Expected ')'")?;
                ret
            }
            _ => self.atom()
        }
    }

    fn atom(&mut self) -> Result<f64, String> {
        match self.parser.peek() {
            '0'..='9' => self.number(),
            _ => self.call()
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        let mut num = 0.;

        while let token = self.parser.peek() {
            match token {
                '0'..='9' => {
                    self.parser.advance();
                    num = num * 10. + token.to_digit(10).unwrap() as f64;
                }
                _ => break
            }
        }

        if self.parser.peek() == '.' {
            self.parser.advance();
            // fraction part
            let mut frac = 0.;
            let mut weight = 0.1;
            while let token = self.parser.peek() {
                match token {
                    '0'..='9' => {
                        self.parser.advance();
                        frac += weight * token.to_digit(10).unwrap() as f64;
                        weight /= 10.;
                    }
                    _ => break
                }
            }
            num += frac;
        }

        Ok(num)
    }

    fn call(&mut self) -> Result<f64, String> {
        match self.parser.peek() {
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.identifier();

                if self.parser.peek() == '(' {
                    self.parser.consume('(', "Expected '('")?;
                    let arguments = self.arguments()?;
                    self.parser.consume(')', "Expected ')'")?;

                    if let Some(func) = self.functions.get(&identifier) {
                        match func {
                            OneArg(f) => {
                                if arguments.len() != 1 {
                                    return Err(format!("Expected 1 argument for function '{}'", identifier));
                                }
                                Ok(f(arguments[0]))
                            },
                            TwoArg(f, ) => {
                                if arguments.len() != 2 {
                                    return Err(format!("Expected 2 arguments for function '{}'", identifier));
                                }
                                Ok(f(arguments[0], arguments[1]))
                            },
                            ThreeArg(f) => {
                                if arguments.len() != 3 {
                                    return Err(format!("Expected 3 arguments for function '{}'", identifier));
                                }
                                Ok(f(arguments[0], arguments[1], arguments[2]))
                            },
                        }
                    } else {
                        Err(format!("Unknown function '{}'", identifier))
                    }
                } else {
                    if let Some(&value) = self.constants.get(&identifier) {
                        Ok(value)
                    } else {
                        Err(format!("Unknown constant '{}'", identifier))
                    }
                }
            }
            _ => {
                Err(format!("Expected a identifier but got {}", self.parser.peek()))
            }
        }
    }

    fn identifier(&mut self) -> String {
        let mut ret = String::new();
        while let token = self.parser.peek() {
            match token {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    ret.push(token);
                    self.parser.advance();
                }
                _ => break
            }
        }
        ret
    }

    fn arguments(&mut self) -> Result<Vec<f64>, String> {
        let mut ret = vec![self.expression()?];
        while let token = self.parser.peek() {
            match token {
                ',' => {
                    self.parser.advance();
                    ret.push(self.expression()?);
                }
                _ => break,
            }
        }
        Ok(ret)
    }
}
