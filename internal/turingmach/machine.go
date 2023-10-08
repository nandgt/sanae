package turingmach

type Program struct {
	Condition int
	Commands  []Token
}

func NewProgram(condition int, cmd string) Program {
	lexer := NewLexer(cmd)
	tokens := lexer.Tokenize()
	return Program{condition, tokens}
}

type TuringMachine struct {
	Tape     []int
	Position int
}

func NewMachine(tape []int) TuringMachine {
	return TuringMachine{tape, 0}
}

func (t TuringMachine) Exec(p Program) {
	awareness := t.Tape[t.Position]
	if p.Condition == awareness {
		for _, cmd := range p.Commands {
			switch cmd.Type {
			case ERASE:
				t.Tape[t.Position] = 0
			case WRITE:
				t.Tape[t.Position] = cmd.Value
			case MOVER:
				t.Position = (t.Position + cmd.Value) % len(t.Tape)
			case MOVEL:
				t.Position = (t.Position - cmd.Value%len(t.Tape) + len(t.Tape)) % len(t.Tape)
			}
		}
	}
}
