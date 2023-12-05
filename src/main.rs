use core::panic;
use std::fs;

#[derive(Debug)]
enum Tokens {
    Eof,
    Erase,
    Goto(i32),
    Halt,
    If(i32),
    Illegal(String),
    Movel(i32),
    Mover(i32),
    Operand(i32),
    Write(i32)
}

struct Lexer<'a> {
    command: Vec<&'a str>,
    pointer: usize,
}

impl<'a> Lexer<'a> {
    fn new(algorithm: &'a str) -> Self {
        let words: Vec<&str> = algorithm
            .trim()
            .split_whitespace()
            .collect();
        Self {
            command: words,
            pointer: 0,
        }
    }

    fn peek_token(&mut self) -> Tokens {
        if self.pointer >= self.command.len() {
            return Tokens::Eof;
        }
        
        let word = self.command[self.pointer];
        self.pointer += 1;
    
        if let Ok(number) = word.parse::<i32>() {
            return Tokens::Operand(number);
        }
    
        match word {
            "if" | "write" | "mover" | "movel" | "goto" => {
                let next_token = self.peek_token();
                if let Tokens::Operand(value) = next_token {
                    match word {
                        "if" => return Tokens::If(value),
                        "write" => return Tokens::Write(value),
                        "mover" => return Tokens::Mover(value),
                        "movel" => return Tokens::Movel(value),
                        "goto" => return Tokens::Goto(value),
                        _ => unreachable!(),
                    };
                } else {
                    panic!("sanae: syntax error: must provide an operand to '{word}'");
                }
            },
            "erase" =>
                return Tokens::Erase,
            "halt" =>
                return Tokens::Halt,
            _ =>
                return Tokens::Illegal(word.to_string()),
        }
    }

    fn tokenize(&mut self) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = Vec::new();
    
        while self.pointer < self.command.len() {
            let current_token = self.peek_token();
            match current_token {
                Tokens::Operand(value) => panic!("sanae: syntax error: '{value}' operand is misplaced"),
                Tokens::Eof => break,
                _ => tokens.push(current_token),
            }
        }
    
        return tokens;
    }
}

// TODO: execute algorithms from cli arguments 
// NOTE(nand): need to find a way to "insert" memory tape into the machine, probably using something like a repl
fn main() {
    let algorithm = fs::read_to_string("algorithms/test.sasm")
        .expect("sanae: could not read file");
    
    let mut lex = Lexer::new(&algorithm);
    let tokens = lex.tokenize();
    println!("{:?}", tokens);
}
