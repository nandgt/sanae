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

struct Parser<'a> {
    lexemes: Vec<&'a str>,
    pointer: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Command {
    condition: i32,
    to_do: Vec<Tokens>,
    in_end: Tokens
}

impl<'a> Parser<'a> {
    fn new(algorithm: &'a str) -> Self {
        let lexemes: Vec<&str> = algorithm
            .trim()
            .split_whitespace()
            .collect();
        Self {
            lexemes,
            pointer: 0,
        }
    }

    fn next_token(&mut self) -> Tokens {
        if self.pointer >= self.lexemes.len() {
            return Tokens::Eof;
        }
        
        let lexeme = self.lexemes[self.pointer];
        self.pointer += 1;
    
        if let Ok(number) = lexeme.parse::<i32>() {
            return Tokens::Operand(number);
        }
    
        match lexeme {
            "if" | "write" | "mover" | "movel" | "goto" => {
                let value = match self.next_token() {
                    Tokens::Operand(value) =>
                        value,
                    _ =>
                        panic!("sanae: syntax error: must provide an operand to '{lexeme}'"),
                };
                match lexeme {
                    "if" =>
                        return Tokens::If(value),
                    "write" =>
                        return Tokens::Write(value),
                    "mover" =>
                        return Tokens::Mover(value),
                    "movel" =>
                        return Tokens::Movel(value),
                    "goto" =>
                        return Tokens::Goto(value),
                    _ =>
                        unreachable!(),
                };
            },
            "erase" =>
                return Tokens::Erase,
            "halt" =>
                return Tokens::Halt,
            _ =>
                return Tokens::Illegal(lexeme.to_string()),
        }
    }

    fn parse_command(&mut self) -> Command {
        let mut to_do: Vec<Tokens> = Vec::new();

        let condition = match self.next_token() {
            Tokens::If(value) =>
                value,
            _ =>
                panic!("sanae: syntax error: every command must start with an if instruction"),
        };

        while self.pointer < self.lexemes.len() - 2 {
            match self.next_token() {
                Tokens::Operand(value) =>
                    panic!("sanae: syntax error: '{value}' operand is misplaced"),
                Tokens::Illegal(lexeme) =>
                    panic!("sanae: syntax error: '{lexeme}' is not a valid instruction"),
                Tokens::Eof =>
                    break,
                token @ _ =>
                    to_do.push(token),
            }
        }

        let in_end = match self.next_token() {
            last_token @ (Tokens::Goto(_) | Tokens::Halt) =>
                last_token,
            _ =>
                panic!("sanae: syntax error: every command must end with a halt or a goto instruction"),
        };

        return Command {
            condition,
            to_do,
            in_end
        };
    }
}

// TODO: execute algorithms from cli arguments 
// NOTE(nand): need to find a way to "insert" memory tape into the machine, probably using something like a repl
fn main() {
    let algorithm = fs::read_to_string("algorithms/test.sasm")
        .expect("sanae: could not read file");
    
    let mut parser = Parser::new(&algorithm);
    let tokens = parser.parse_command();
    println!("{:?}", tokens);
}
