package main

import (
	"fmt"

	"github.com/nobytesguy/sanae/internal/sanae"
)

func main() {
	reader := sanae.NewReader("algorithms/test.sasm")
	algorithm := reader.Read()

	tape := []int{1, 2, 3, 4, 5, 6, 7}
	machine := sanae.NewMachine(tape, algorithm)

	fmt.Println(machine.Tape)
	machine.Exec()
	fmt.Println(machine.Tape)
}
