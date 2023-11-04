pub(crate) struct Parser {
    index: usize,
    expr: Vec<char>,
}

impl Parser {
    pub fn new(expr: String) -> Self {
        Parser {
            index: 0,
            expr: expr.chars().filter(|&c| c != ' ').collect(),
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn peek(&self) -> char {
        if self.index >= self.expr.len() {
            '\0'
        } else {
            self.expr[self.index]
        }
    }

    pub fn consume(&mut self, c: char, msg: &str) -> Result<(), String> {
        if self.peek() == c {
            self.advance();
            Ok(())
        } else {
            Err(msg.to_string())
        }
    }
}
