package sanae

type SanaeMachine struct {
	Tape     []int
	Position int
	Algorithm
}

func NewMachine(tape []int, algorithm Algorithm) SanaeMachine {
	return SanaeMachine{
		Tape:      tape,
		Position:  0,
		Algorithm: algorithm,
	}
}

func (s SanaeMachine) Exec() {
	var currentState int
	for currentState < len(s.Algorithm.States) {
		state := s.Algorithm.getState(currentState)
		awareness := s.Tape[s.Position]
		cmd, exists := state.Commands[awareness]
		if !exists {
			currentState++
			continue
		}
		for _, instruction := range cmd.Instructions {
			switch instruction.Type {
			case ERASE:
				s.Tape[s.Position] = 0
			case WRITE:
				s.Tape[s.Position] = instruction.Value
			case MOVER:
				s.Position = (s.Position + instruction.Value) % len(s.Tape)
			case MOVEL:
				s.Position = (s.Position - instruction.Value%len(s.Tape) + len(s.Tape)) % len(s.Tape)
			case HALT:
				return
			}
		}
		currentState = cmd.NextState
	}
}
