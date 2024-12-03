package main

import (
	"adventofcode/day01"
	"adventofcode/day02"
	"fmt"
	"os"
)

func main() {
	args := os.Args[1:]
	fmt.Println("running with args", args)

	// TODO make this dynamic based on arg, run one day, or all days, etc
	day01.Solve("day01")
	day02.Solve("day02")
}
