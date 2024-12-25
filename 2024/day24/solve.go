package day24

import (
	"adventofcode/util"
	"regexp"
	"slices"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(69201640933606, part1(input))
	util.AssertEqual(0, part2(input))
}

type Input struct {
	initial map[string]bool
	gates   map[string]Gate
	zNames  []string
}

type Gate struct {
	left, operation, right, output string
}

func parseInput(input string) Input {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	// parse initial vals
	initial := make(map[string]bool, 0)
	for _, line := range strings.Split(parts[0], "\n") {
		lineParts := strings.Split(line, ": ")
		util.AssertEqual(2, len(lineParts))

		switch lineParts[1] {
		case "0":
			initial[lineParts[0]] = false
		case "1":
			initial[lineParts[0]] = true
		default:
			panic("Invalid value")
		}
	}

	zNames := make([]string, 0)

	// parse gates
	gates := make(map[string]Gate, 0)
	re := regexp.MustCompile(`\A(\w+) (\w+) (\w+) \-> (\w+)\z`)
	for _, line := range strings.Split(parts[1], "\n") {
		match := re.FindStringSubmatch(line)
		left := match[1]
		operation := match[2]
		right := match[3]
		output := match[4]
		gates[output] = Gate{left, operation, right, output}

		if output[0] == 'z' {
			zNames = append(zNames, output)
		}
	}

	slices.Sort(zNames)

	return Input{initial, gates, zNames}
}

func (g *Gate) apply(left, right bool) bool {
	switch g.operation {
	case "AND":
		return left && right
	case "OR":
		return left || right
	case "XOR":
		return left != right
	default:
		panic("Unreachable")
	}
}

func calculateOutput(name string, initial map[string]bool, gates map[string]Gate) bool {
	if val, ok := initial[name]; ok {
		return val
	}

	if gate, ok := gates[name]; ok {
		// calculate the value of the gate
		left := calculateOutput(gate.left, initial, gates)
		right := calculateOutput(gate.right, initial, gates)
		return gate.apply(left, right)
	}

	panic("Unreachable")
}

func part1(input Input) int {
	// figure out the values for z gates
	zValues := make(map[string]bool, len(input.zNames))
	for _, name := range input.zNames {
		zValues[name] = calculateOutput(name, input.initial, input.gates)
	}

	// assuming all z vals between 0 and N are accounted for, no gaps
	res := 0
	for i := len(input.zNames) - 1; i >= 0; i-- {
		name := input.zNames[i]
		res <<= 1
		if zValues[name] {
			res |= 1
		}
	}
	return res
}

func part2(input Input) int {
	return 0
}
