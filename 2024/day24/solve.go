package day24

import (
	"adventofcode/set"
	"adventofcode/util"
	"fmt"
	"regexp"
	"slices"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(69201640933606, part1(input))
	util.AssertEqual("dhq,hbs,jcp,kfp,pdg,z18,z22,z27", part2(input))
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

type AdderStep struct {
	val, output, localCarry, extraCarry, carry string
}

func findGate(left, right, operation string, remaining *set.Set[string], gates map[string]Gate) (string, bool) {
	for name := range remaining.Iter() {
		gate := gates[name]
		if operation == gate.operation && ((gate.left == left && gate.right == right) || (gate.left == right && gate.right == left)) {
			return name, true
		}
	}
	return "", false
}

func findBadInput(gate Gate, a, b string) (string, string) {
	if gate.left == a {
		util.AssertEqual(false, gate.right == b)
		return gate.right, b
	} else if gate.right == a {
		util.AssertEqual(false, gate.left == b)
		return gate.left, b
	} else if gate.left == b {
		util.AssertEqual(false, gate.right == a)
		return gate.right, a
	} else if gate.right == b {
		util.AssertEqual(false, gate.left == b)
		return gate.left, a
	} else {
		panic("both inputs are bad")
	}
}

func repairAdder(input *Input) [][2]string {
	remaining := set.NewSet[string]()
	for name := range input.gates {
		remaining.Add(name)
	}

	gates := input.gates

	swaps := [][2]string{}

	adder := make([]AdderStep, len(input.zNames))

	for i, output := range input.zNames {
		// check and fix
		first := i == 0
		last := i == len(input.zNames)-1
		carry := ""
		if i > 0 {
			carry = adder[i-1].carry
		}

		// keep attempting to fix this step
		stepOk := false
		for !stepOk {
			step, swap, ok := repairAdderStep(output, carry, first, last, &remaining, gates)
			if ok {
				// yay, it's fixed!
				stepOk = true

				// cleanup
				remaining.Remove(step.val)
				remaining.Remove(step.output)
				remaining.Remove(step.localCarry)
				remaining.Remove(step.extraCarry)
				remaining.Remove(step.carry)

				adder[i] = step
			} else {
				// not fixed yet -- we need a swap
				name1, name2 := swap[0], swap[1]
				util.AssertEqual(true, remaining.Contains(name1))
				util.AssertEqual(true, remaining.Contains(name2))
				gate1, gate2 := gates[name1], gates[name2]
				gates[name1] = gate2
				gates[name2] = gate1

				// and record it
				swaps = append(swaps, swap)
			}
		}
	}

	return swaps
}

// return false if we did a swap and should retry this step
func repairAdderStep(output, prevCarry string, first, last bool, remaining *set.Set[string], gates map[string]Gate) (AdderStep, [2]string, bool) {
	if last {
		// special case for last step, no value or localCarry, just the residual carry
		// once 1 gate:
		// - will be the previous carry!
		util.AssertEqual(output, prevCarry)
		step := AdderStep{output: output}
		util.AssertEqual(0, remaining.Len())
		return step, [2]string{}, true
	}

	// val and carry are easiest
	x := strings.Replace(output, "z", "x", 1)
	y := strings.Replace(output, "z", "y", 1)
	val, ok := findGate(x, y, "XOR", remaining, gates)
	if !ok {
		panic(fmt.Sprintf("missing val gate: %s XOR %s", x, y))
	}
	localCarry, ok := findGate(x, y, "AND", remaining, gates)
	if !ok {
		panic(fmt.Sprintf("missing localCarry gate: %s AND %s", x, y))
	}

	if first {
		// special case for the first step, only 2 gates instead of 5
		// - val -> result
		// - localCarry -> carry
		util.AssertEqual(output, val)
		step := AdderStep{val: val, carry: localCarry, output: output}
		return step, [2]string{}, true
	} else {
		// 5 gates total:
		// - 2 used to calculate the output
		//   - x XOR y -> val
		//   - val XOR prevCarry -> output
		// - 3 used to calculate the carry
		//   - x AND y -> localCarry
		//   - val AND prevCarry -> extraCarry
		//   - localCarry OR extraCarry -> carry

		// locate the result gate
		result, ok := findGate(val, prevCarry, "XOR", remaining, gates)
		if !ok {
			// couldn't find it via the value, look it up by name and try to repair

			// find it via the z name
			result := gates[output]
			util.AssertEqual("XOR", result.operation)

			// one of the two inputs is wrong
			// is it the val?
			a, b := findBadInput(result, val, prevCarry)

			// return a swap, caller will apply and retry
			return AdderStep{}, [2]string{a, b}, false
		}

		if output != result {
			// we found the correct result gate, but the output wire needs to be swapped
			return AdderStep{}, [2]string{output, result}, false
		}

		extraCarry, ok := findGate(val, prevCarry, "AND", remaining, gates)
		if !ok {
			panic(fmt.Sprintf("missing extraCarry gate: %s AND %s", x, y))
		}

		carry, ok := findGate(localCarry, extraCarry, "OR", remaining, gates)
		if !ok {
			panic(fmt.Sprintf("missing carry gate: %s OR %s", x, y))
		}

		step := AdderStep{val, output, localCarry, extraCarry, carry}
		return step, [2]string{}, true
	}
}

func part2(input Input) string {
	swaps := repairAdder(&input)

	names := []string{}
	for _, swap := range swaps {
		names = append(names, swap[0], swap[1])
	}
	slices.Sort(names)

	return strings.Join(names, ",")
}
