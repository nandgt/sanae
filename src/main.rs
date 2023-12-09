use core::panic;
use std::fs;

#[derive(Debug)]
enum Token {
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

// TODO: improve this
struct Parser<'a> {
    lexemes: Vec<&'a str>,
    pointer: usize
}

#[derive(Debug)]
#[allow(dead_code)]
struct Command {
    to_do: Vec<Token>,
    in_end: Token
}

struct Machine {
    tape: Vec<i32>,
    index: usize
}

impl<'a> Parser<'a> {
    fn new() -> Self {
        Self {
            lexemes: vec![],
            pointer: 0
        }
    }

    fn next_token(&mut self) -> Token {
        if self.pointer >= self.lexemes.len() {
            return Token::Eof;
        }
        
        let lexeme = self.lexemes[self.pointer];
        self.pointer += 1;
    
        if let Ok(number) = lexeme.parse::<i32>() {
            return Token::Operand(number);
        }
    
        match lexeme {
            "if" | "write" | "mover" | "movel" | "goto" => {
                let value = match self.next_token() {
                    Token::Operand(value) => value,
                    _ => panic!("sanae: syntax error: must provide an operand to {} instruction", lexeme),
                };
                match lexeme {
                    "if" => return Token::If(value),
                    "write" => return Token::Write(value),
                    "mover" => return Token::Mover(value),
                    "movel" => return Token::Movel(value),
                    "goto" => return Token::Goto(value),
                    _ => unreachable!(),
                };
            },
            "erase" => return Token::Erase,
            "halt" => return Token::Halt,
            _ => return Token::Illegal(lexeme.to_string()),
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = self.next_token();
            match token {
                Token::Eof => break,
                Token::Operand(_) => panic!("sanae: syntax error: {:?} is misplaced", token),
                Token::Illegal(lexeme) =>
                    panic!("sanae: syntax error: '{}' is not a valid instruction", lexeme),
                _ => tokens.push(token),
            }
        }

        return tokens;
    }

    fn parse_command(&mut self) -> Command {
        let mut tokens = self.tokenize();

        for token in &tokens[1 .. tokens.len() - 1] {
            match token {
                Token::If(_) =>
                    panic!("sanae: syntax error: {:?} instruction can only be at the command start", token),
                Token::Halt | Token::Goto(_) =>
                    panic!("sanae: syntax error: {:?} instruction can only be at the command end", token),
                _ => ()
            }
        }

        let in_end = match tokens.pop().unwrap() {
            last_token @ (Token::Goto(_) | Token::Halt) => last_token,
            _ => panic!("sanae: syntax error: every command must end with a Halt or a Goto instruction"),
        };

        return Command {
            to_do: tokens,
            in_end
        };
    }

    fn parse_algorithm(&mut self, algorithm: &'a str) -> Vec<Command> {
        let mut commands: Vec<Command> = vec![];
        let lines: Vec<&str> = algorithm.lines().collect();

        for line in lines.iter() {
            self.lexemes = line.split_whitespace().collect();
            commands.push(self.parse_command());
            self.pointer = 0;
        }

        return commands
    }
}

impl Machine {
    fn new(tape: Vec<i32>) -> Self {
        Self {
            tape,
            index: 0
        }
    }

    fn execute_instruction(&mut self, instruction: &Token) {
        match instruction {
            Token::Write(value) => self.tape[self.index] = *value,
            Token::Erase => self.tape[self.index] = 0,
            Token::Movel(value) => self.index += *value as usize,
            Token::Mover(value) => self.index -= *value as usize,
            _ => panic!("sanae: runtime error: {:?} isn't a known instruction", instruction)
        }
    }
    
    fn execute_algorithm(&mut self, mut commands: Vec<Command>) {
        while self.index < commands.len() {
            let command = &mut commands[self.index];
            if let Token::If(value) = command.to_do[0] {
                command.to_do.remove(0);
                if self.tape[self.index] != value {
                    self.index += 1;
                    continue;
                }
            }
            command.to_do
                .iter()
                .for_each(|instruction| self.execute_instruction(instruction));
            match command.in_end {
                Token::Goto(value) => self.index = (value - 1) as usize,
                Token::Halt => break,
                _ => unreachable!()
            }
        }
    }
}

// TODO: execute algorithms from cli arguments 
// NOTE(nand): need to find a way to "insert" memory tape into the machine, probably using something like a repl
fn main() {
    let algorithm = fs::read_to_string("algorithms/test.sasm")
        .expect("sanae: could not read file");

    let mut machine = Machine::new(vec![1, 2, 3, 4, 5]);
    let mut parser = Parser::new();

    let commands = parser.parse_algorithm(&algorithm);
    machine.execute_algorithm(commands);
    println!("{:?}", machine.tape);
}
