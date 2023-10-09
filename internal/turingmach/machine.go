package turingmach

type Command struct {
	Condition    int
	Instructions []Token
	NextState    int
}

func NewCommand(condition int, text string, nextState int) Command {
	lexer := NewLexer(text)
	instructions := lexer.Tokenize()
	return Command{condition, instructions, nextState}
}

type State struct {
	Commands []Command
}

func NewState(commands []Command) State {
	return State{commands}
}

type Algorithm struct {
	States []State
}

func NewAlgorithm(states []State) Algorithm {
	return Algorithm{states}
}

type TuringMachine struct {
	Tape     []int
	Position int
}

func NewMachine(tape []int) TuringMachine {
	return TuringMachine{tape, 0}
}

func (t TuringMachine) Exec(a Algorithm) {
	for _, state := range a.States {
		for _, cmd := range state.Commands {
			awareness := t.Tape[t.Position]
			if cmd.Condition != awareness {
				continue
			}
			for _, instruction := range cmd.Instructions {
				switch instruction.Type {
				case ERASE:
					t.Tape[t.Position] = 0
				case WRITE:
					t.Tape[t.Position] = instruction.Value
				case MOVER:
					t.Position = (t.Position + instruction.Value) % len(t.Tape)
				case MOVEL:
					t.Position = (t.Position - instruction.Value%len(t.Tape) + len(t.Tape)) % len(t.Tape)
				case HALT:
					return
				}
			}
		}
	}
}
