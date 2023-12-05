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

fn peek_token(words: &[&str], index: &mut usize) -> Tokens {
    if *index >= words.len() {
        return Tokens::Eof;
    }
    
    let word = words[*index];
    *index += 1;

    if let Ok(number) = word.parse::<i32>() {
        return Tokens::Operand(number);
    }

    match word {
        "if" | "write" | "mover" | "movel" | "goto" => {
            let next_token = peek_token(words, index);
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

fn tokenize(algorithm: String, index: &mut usize) -> Vec<Tokens> {
    let words: Vec<&str> = algorithm
        .trim()
        .split_whitespace()
        .collect();
    let mut tokens: Vec<Tokens> = Vec::new();

    while *index < words.len() {
        let current_token = peek_token(&words, index);
        match current_token {
            Tokens::Operand(value) => panic!("sanae: syntax error: '{value}' operand is misplaced"),
            Tokens::Eof => break,
            Tokens::Illegal(word) => panic!("sanae: syntax error: '{word}' is not a valid instruction"),
            _ => tokens.push(current_token),
        }
    }

    return tokens;
}

fn main() {
    let algorithm = fs::read_to_string("algorithms/test.sasm")
        .expect("sanae: could not read file");
    let mut index = 0;
    let tokens = tokenize(algorithm, &mut index);
    println!("{:?}", tokens);
}
