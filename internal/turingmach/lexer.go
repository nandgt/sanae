package turingmach

import (
	"fmt"
	"strconv"
	"strings"
	"unicode"
)

type TokenType string

const (
	ERASE   TokenType = "ERASE"
	WRITE   TokenType = "WRITE"
	MOVER   TokenType = "MOVER"
	MOVEL   TokenType = "MOVEL"
	HALT    TokenType = "HALT"
	OPERAND TokenType = "OPERAND"
)

type Token struct {
	Type  TokenType
	Value int
}

type Lexer struct {
	Text          string
	Position      int
	PreviousToken Token
}

func NewLexer(cmd string) Lexer {
	return Lexer{cmd, 0, Token{}}
}

func (l *Lexer) getCurrentWord() string {
	var word string
	for l.Position < len(l.Text) {
		char := rune(l.Text[l.Position])
		if unicode.IsSpace(char) {
			l.Position--
			break
		}
		word += string(char)
		l.Position++
	}
	return word
}

func (l *Lexer) tokenizeOperand() Token {
	word := l.getCurrentWord()
	number, err := strconv.Atoi(word)
	if err != nil {
		panic(err)
	}
	token := Token{OPERAND, number}
	l.PreviousToken = token
	return token
}

func (l *Lexer) tokenizeInstruction() Token {
	word := l.getCurrentWord()
	var token Token
	switch strings.ToLower(word) {
	case "write":
		token = Token{WRITE, 0}
	case "erase":
		token = Token{ERASE, 0}
	case "mover":
		token = Token{MOVER, 1}
	case "movel":
		token = Token{MOVEL, 1}
	case "halt":
		token = Token{HALT, 0}
	default:
		panic(fmt.Errorf("syntax error: %s isn't an instruction", word))
	}
	l.PreviousToken = token
	return token
}

func (l *Lexer) Tokenize() []Token {
	var tokens []Token
	for l.Position < len(l.Text) {
		char := rune(l.Text[l.Position])
		if unicode.IsLetter(char) {
			tokens = append(tokens, l.tokenizeInstruction())
		} else if unicode.IsDigit(char) {
			switch previousToken := l.PreviousToken; previousToken.Type {
			case WRITE, MOVER, MOVEL:
				tokens[len(tokens)-1].Value = l.tokenizeOperand().Value
			default:
				panic(fmt.Errorf("syntax error: %s doesn't accept operands", previousToken.Type))
			}
		} else if !unicode.IsSpace(char) {
			panic(fmt.Errorf("syntax error: %s unknown character", string(char)))
		}
		l.Position++
	}
	return tokens
}
