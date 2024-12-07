package day07

import (
	"adventofcode/util"
	"fmt"
	"strings"

	"github.com/gammazero/deque"
	"github.com/samber/lo"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	fmt.Println("part 1: ", part1(input))
	fmt.Println("part 2: ", part2(input))
}

func parseInput(input string) []Equation {
	lines := strings.Split(input, "\n")
	return lo.Map[string](lines, func(s string, _ int) Equation {
		return parseEquation(s)
	})
}

type Equation struct {
	result int
	values []int
}

func parseEquation(input string) Equation {
	parts := strings.Split(input, ": ")
	util.AssertEqual(2, len(parts))

	result := util.StringToInt(parts[0])
	values := lo.Map(strings.Fields(parts[1]), func(s string, _ int) int { return util.StringToInt(s) })

	return Equation{result, values}
}

type SolutionState struct {
	equation *Equation
	index    int
	acc      int
}

func solveEquation(equation *Equation) bool {
	var q deque.Deque[SolutionState]

	initialState := SolutionState{
		equation: equation,
		index:    1,
		acc:      equation.values[0],
	}
	q.PushBack(initialState)

	for q.Len() != 0 {
		state := q.PopFront()

		if state.index == len(equation.values) {
			if state.acc == equation.result {
				// found a solution
				return true
			}
			// otherwise, ignore this one
		} else {
			val := equation.values[state.index]

			// try add
			added := state.acc + val
			if added <= equation.result {
				q.PushFront(SolutionState{equation: equation, index: state.index + 1, acc: added})
			}

			// try multiply
			multiplied := state.acc * val
			if multiplied <= equation.result {
				q.PushFront(SolutionState{equation: equation, index: state.index + 1, acc: multiplied})
			}
		}
	}

	// no solution found
	return false
}

func part1(input string) int {
	equations := parseInput(input)

	solved := lo.Filter(equations, func(eq Equation, _ int) bool { return solveEquation(&eq) })
	return lo.SumBy(solved, func(eq Equation) int { return eq.result })
}

func part2(input string) int {
	return 0
}
