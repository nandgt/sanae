package main

import (
	"fmt"

	"github.com/nobytesguy/sanae/internal/sanae"
)

func main() {
	firstCmd := sanae.NewCommand(1, "write 2 mover", 1)
	secondCmd := sanae.NewCommand(2, "write 4 mover", 1)
	firstState := sanae.NewState([]sanae.Command{firstCmd, secondCmd})

	firstCmd = sanae.NewCommand(2, "write 4 halt", 0)
	secondCmd = sanae.NewCommand(4, "write 8 halt", 0)
	secondState := sanae.NewState([]sanae.Command{firstCmd, secondCmd})

	algorithm := sanae.NewAlgorithm([]sanae.State{firstState, secondState})

	tape := []int{1, 2, 3, 4, 5, 6, 7}
	machine := sanae.NewMachine(tape, algorithm)

	fmt.Println(machine.Tape)
	machine.Exec()
	fmt.Println(machine.Tape)
}
