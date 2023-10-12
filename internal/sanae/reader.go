package sanae

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type TokenType string

const (
	IF      TokenType = "IF"
	ERASE   TokenType = "ERASE"
	WRITE   TokenType = "WRITE"
	MOVER   TokenType = "MOVER"
	MOVEL   TokenType = "MOVEL"
	GOTO    TokenType = "GOTO"
	HALT    TokenType = "HALT"
	OPERAND TokenType = "OPERAND"
)

type Token struct {
	Type  TokenType
	Value int
}

func toToken(word string) Token {
	if operand, err := strconv.Atoi(word); err == nil {
		return Token{
			Type:  OPERAND,
			Value: operand,
		}
	}
	switch strings.ToLower(word) {
	case "if":
		return Token{
			Type:  IF,
			Value: 0,
		}
	case "erase":
		return Token{
			Type:  ERASE,
			Value: 0,
		}
	case "write":
		return Token{
			Type:  WRITE,
			Value: 0,
		}
	case "mover":
		return Token{
			Type:  MOVER,
			Value: 0,
		}
	case "movel":
		return Token{
			Type:  MOVEL,
			Value: 0,
		}
	case "goto":
		return Token{
			Type:  GOTO,
			Value: 0,
		}
	case "halt":
		return Token{
			Type:  HALT,
			Value: 0,
		}
	default:
		panic(fmt.Errorf("syntax error: %s isn't an instruction", word))
	}
}

type Command struct {
	Condition    int
	Instructions []Token
	NextState    int
}

func isValidCommand(instructions []Token) bool {
	var hasCondition, hasEnd bool
	for _, instruction := range instructions {
		switch instruction.Type {
		case IF:
			hasCondition = true
		case GOTO, HALT:
			hasEnd = true
		}
	}
	return hasCondition && hasEnd
}

func newCommand(instructions []Token) Command {
	if !isValidCommand(instructions) {
		panic(fmt.Errorf("syntax error: not a valid command"))
	}
	command := Command{}
	for _, instruction := range instructions {
		switch instruction.Type {
		case IF:
			command.Condition = instruction.Value
		case GOTO:
			command.NextState = instruction.Value
		default:
			command.Instructions = append(command.Instructions, instruction)
		}
	}
	return command
}

type State struct {
	Commands map[int]Command
}

func NewState() State {
	return State{
		Commands: make(map[int]Command),
	}
}

func (s *State) addCommand(cmd Command) {
	s.Commands[cmd.Condition] = cmd
}

type Algorithm struct {
	States []State
}

func (a *Algorithm) addState(state State) {
	a.States = append(a.States, state)
}

func (a Algorithm) getState(index int) State {
	return a.States[index]
}

type Reader struct {
	File     *os.File
	Scanner  *bufio.Scanner
	Position int
}

func NewReader(filePath string) Reader {
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}
	return Reader{
		File:     file,
		Scanner:  bufio.NewScanner(file),
		Position: 0,
	}
}

func (r *Reader) TokenizeCommand(line string) []Token {
	var tokens []Token
	words := strings.Split(line, " ")

	for r.Position < len(words) {
		word := words[r.Position]
		token := toToken(word)

		switch token.Type {
		case WRITE, MOVER, MOVEL, IF, GOTO:
			r.Position++
			if r.Position >= len(words) {
				panic(fmt.Errorf("syntax error: %s instruction needs an operand", word))
			}

			nextWord := words[r.Position]
			operand, err := strconv.Atoi(nextWord)
			if err != nil {
				panic(fmt.Errorf("syntax error: %s instruction needs an operand", word))
			}
			token.Value = operand
		case OPERAND:
			panic(fmt.Errorf("syntax error: %s misplaced operand", word))
		}

		tokens = append(tokens, token)
		r.Position++
	}
	r.Position = 0
	return tokens
}

func (r *Reader) Read() Algorithm {
	defer r.File.Close()

	algorithm := Algorithm{}
	algorithm.addState(NewState())
	var stateCount int

	for r.Scanner.Scan() {
		line := r.Scanner.Text()
		if len(line) == 0 {
			algorithm.addState(NewState())
			stateCount++
			continue
		}

		state := algorithm.getState(stateCount)
		instructions := r.TokenizeCommand(line)
		state.addCommand(newCommand(instructions))
	}
	return algorithm
}
