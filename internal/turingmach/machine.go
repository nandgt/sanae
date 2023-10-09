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
	Commands map[int]Command
}

func NewState(commands []Command) State {
	state := State{make(map[int]Command)}
	for _, cmd := range commands {
		state.Commands[cmd.Condition] = cmd
	}
	return state
}

type Algorithm struct {
	States []State
}

func NewAlgorithm(states []State) Algorithm {
	return Algorithm{states}
}

type TuringMachine struct {
	Tape      []int
	Position  int
	Algorithm Algorithm
}

func NewMachine(tape []int, algorithm Algorithm) TuringMachine {
	return TuringMachine{tape, 0, algorithm}
}

func (t TuringMachine) Exec() {
	var currentState int
	for currentState < len(t.Algorithm.States) {
		state := t.Algorithm.States[currentState]
		awareness := t.Tape[t.Position]
		cmd, exists := state.Commands[awareness]
		if !exists {
			currentState++
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
		currentState = cmd.NextState
	}
}
