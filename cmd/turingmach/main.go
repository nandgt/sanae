package main

import (
	"fmt"

	"github.com/nobytesguy/turing_machine/internal/turingmach"
)

func main() {
	tape := []int{1, 2, 3, 4, 5, 6, 7}
	machine := turingmach.NewMachine(tape)

	firstCmd := turingmach.NewCommand(1, "write 2 mover", 1)
	secondCmd := turingmach.NewCommand(2, "write 4 mover", 1)
	firstState := turingmach.NewState([]turingmach.Command{firstCmd, secondCmd})

	firstCmd = turingmach.NewCommand(2, "write 4 halt", 0)
	secondCmd = turingmach.NewCommand(4, "write 8 halt", 0)
	secondState := turingmach.NewState([]turingmach.Command{firstCmd, secondCmd})

	algorithm := turingmach.NewAlgorithm([]turingmach.State{firstState, secondState})

	fmt.Println(machine.Tape)
	machine.Exec(algorithm)
	fmt.Println(machine.Tape)
}
