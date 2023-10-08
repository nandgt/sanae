package main

import (
	"fmt"
	"turingmach/internal/turingmach"
)

func main() {
	cmd := "write 69 mover write 69 mover write 69 mover write 69 mover write 69 mover write 69 mover write 69"
	tape := []int{1, 2, 3, 4, 5, 6, 7}

	machine := turingmach.NewMachine(tape)
	program := turingmach.NewProgram(1, cmd)

	fmt.Println(machine.Tape)
	machine.Exec(program)
	fmt.Println(machine.Tape)
}
