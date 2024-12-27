package day25

import (
	"adventofcode/util"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(3155, part1(input))
}

type Input struct {
	locks [][5]int
	keys  [][5]int
}

func parseInput(input string) Input {
	chunks := strings.Split(input, "\n\n")

	locks := [][5]int{}
	keys := [][5]int{}

	for _, chunk := range chunks {
		lines := strings.Split(chunk, "\n")
		util.AssertEqual(7, len(lines))
		if lines[0] == "#####" {
			locks = append(locks, parseLock(lines))
		} else if lines[0] == "....." {
			keys = append(keys, parseKey(lines))
		} else {
			panic("Unreachable")
		}
	}

	return Input{locks, keys}
}

func parseLock(lines []string) [5]int {
	res := [5]int{}
	for x := 0; x < 5; x++ {
		for y, line := range lines {
			if line[x] == '.' {
				res[x] = y - 1
				break
			}
		}
	}
	return res
}

func parseKey(lines []string) [5]int {
	res := [5]int{}
	for x := 0; x < 5; x++ {
		for y, line := range lines {
			if line[x] == '#' {
				res[x] = len(lines) - y - 1
				break
			}
		}
	}
	return res
}

func part1(input Input) int {
	res := 0

	for _, key := range input.keys {
		for _, lock := range input.locks {
			all := true
			for x := 0; x < 5; x++ {
				if key[x]+lock[x] > 5 {
					all = false
					break
				}
			}

			if all {
				res++
			}
		}
	}

	return res
}
