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

// TODO: improve this
struct Parser<'a> {
    algorithm: &'a str,
    lexemes: Vec<&'a str>,
    pointer: usize
}

#[derive(Debug)]
#[allow(dead_code)]
struct Command {
    condition: i32,
    to_do: Vec<Tokens>,
    in_end: Tokens
}

struct Machine {
    tape: Vec<i32>,
    index: usize
}

impl<'a> Parser<'a> {
    // TODO: improve this
    fn new(algorithm: &'a str) -> Self {
        Self {
            algorithm,
            lexemes: Vec::new(),
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
                    Tokens::Operand(value) => value,
                    _ => panic!("sanae: syntax error: must provide an operand to {} instruction", lexeme),
                };
                match lexeme {
                    "if" => return Tokens::If(value),
                    "write" => return Tokens::Write(value),
                    "mover" => return Tokens::Mover(value),
                    "movel" => return Tokens::Movel(value),
                    "goto" => return Tokens::Goto(value),
                    _ => unreachable!(),
                };
            },
            "erase" => return Tokens::Erase,
            "halt" => return Tokens::Halt,
            _ => return Tokens::Illegal(lexeme.to_string()),
        }
    }

    fn parse_command(&mut self) -> Command {
        let mut to_do: Vec<Tokens> = Vec::new();

        let condition = match self.next_token() {
            Tokens::If(value) => value,
            _ => panic!("sanae: syntax error: every command must start with an If instruction"),
        };

        while self.pointer < self.lexemes.len() - 2 {
            let token = self.next_token();
            match token {
                Tokens::Operand(_) => panic!("sanae: syntax error: {:?} is misplaced", token),
                Tokens::Illegal(lexeme) => panic!("sanae: syntax error: '{}' is not a valid instruction", lexeme),
                Tokens::If(_) => panic!("sanae: syntax error: {:?} instruction can only be at the command start", token),
                Tokens::Halt | Tokens::Goto(_) => panic!("sanae: syntax error: {:?} instruction can only be at the command end", token),
                Tokens::Eof => break,
                _ => to_do.push(token),
            }
        }

        let in_end = match self.next_token() {
            last_token @ (Tokens::Goto(_) | Tokens::Halt) => last_token,
            _ => panic!("sanae: syntax error: every command must end with a Halt or a Goto instruction"),
        };

        return Command {
            condition,
            to_do,
            in_end
        };
    }

    // TODO: improve this
    fn parse_algorithm(&mut self) -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
        let lines: Vec<Vec<&str>> = self.algorithm
            .lines()
            .map(|x| x.split_whitespace().collect())
            .collect();

        for line in lines.iter() {
            self.lexemes = line.to_vec();
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

    fn execute_instruction(&mut self, instruction: &Tokens) {
        match instruction {
            Tokens::Write(value) => self.tape[self.index] = *value,
            Tokens::Erase => self.tape[self.index] = 0,
            Tokens::Movel(value) => self.index += *value as usize,
            Tokens::Mover(value) => self.index -= *value as usize,
            _ => panic!("sanae: runtime error: {:?} isn't a known instruction", instruction)
        }
    }
    
    fn execute_algorithm(&mut self, commands: Vec<Command>) {
        while self.index < commands.len() {
            let command = &commands[self.index];
            if command.condition != self.tape[self.index] {
                break;
            }
            command.to_do.iter().for_each(|instruction| self.execute_instruction(instruction));
            match command.in_end {
                Tokens::Goto(value) => self.index = (value - 1) as usize,
                Tokens::Halt => break,
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
    let mut parser = Parser::new(&algorithm);

    let commands = parser.parse_algorithm();
    machine.execute_algorithm(commands);
    println!("{:?}", machine.tape);
}
