package day03

import (
	"adventofcode/util"
	"regexp"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(187194524, part1(input))
	util.AssertEqual(127092535, part2(input))
}

// just here for consistency
func parseInput(input string) string {
	return input
}

type Instruction struct {
	left  int
	right int
}

func parseInstructions(input string, useConditionals bool) []Instruction {
	re := regexp.MustCompile(`(mul|do|don't)\(((\d{1,3}),(\d{1,3}))?\)`)

	matches := re.FindAllStringSubmatch(input, -1)

	ignoreMul := false

	instructions := make([]Instruction, 0)
	for _, match := range matches {
		kind := match[1]
		switch kind {
		case "mul":
			if !ignoreMul && len(match[2]) > 0 {
				left := util.StringToInt(match[3])
				right := util.StringToInt(match[4])
				instructions = append(instructions, Instruction{left, right})
			}
		case "do":
			ignoreMul = false
		case "don't":
			if useConditionals {
				ignoreMul = true
			}
		default:
			panic("Unreachable")
		}
	}
	return instructions
}

func applyInstructions(instructions []Instruction) int {
	result := 0
	for _, instruction := range instructions {
		result += instruction.left * instruction.right
	}
	return result
}

func part1(input string) int {
	instructions := parseInstructions(input, false)
	return applyInstructions(instructions)
}

func part2(input string) int {
	instructions := parseInstructions(input, true)
	return applyInstructions(instructions)
}
