package main

import (
	"adventofcode/day01"
	"adventofcode/day02"
	"adventofcode/day03"
	"adventofcode/day04"
	"adventofcode/day05"
	"adventofcode/day06"
	"adventofcode/day07"
	"adventofcode/day08"
	"adventofcode/day09"
	"fmt"
	"os"
)

func main() {
	args := os.Args[1:]
	fmt.Println("running with args", args)

	// TODO make this dynamic based on arg, run one day, or all days, etc
	day01.Solve("day01")
	day02.Solve("day02")
	day03.Solve("day03")
	day04.Solve("day04")
	day05.Solve("day05")
	day06.Solve("day06")
	day07.Solve("day07")
	day08.Solve("day08")
	day09.Solve("day09")
}
