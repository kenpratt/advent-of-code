package day07

import (
	"adventofcode/stack"
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(2501605301465, part1(input))
	util.AssertEqual(44841372855953, part2(input))
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

func solveEquation(equation *Equation, enableConcatenation bool, st *stack.Stack[SolutionState]) bool {
	initialState := SolutionState{
		equation: equation,
		index:    1,
		acc:      equation.values[0],
	}
	st.Push(initialState)

	for st.Len() != 0 {
		state := st.Pop()

		if state.index == len(equation.values) {
			if state.acc == equation.result {
				// found a solution
				st.Clear()
				return true
			}
			// otherwise, ignore this one
		} else {
			val := equation.values[state.index]

			// try add
			added := state.acc + val
			if added <= equation.result {
				st.Push(SolutionState{equation: equation, index: state.index + 1, acc: added})
			}

			// try multiply
			multiplied := state.acc * val
			if multiplied <= equation.result {
				st.Push(SolutionState{equation: equation, index: state.index + 1, acc: multiplied})
			}

			if enableConcatenation {
				// try concatenation
				concatenated := concatenate(state.acc, val)
				if concatenated <= equation.result {
					st.Push(SolutionState{equation: equation, index: state.index + 1, acc: concatenated})
				}
			}
		}
	}

	// no solution found
	return false
}

func concatenate(l, r int) int {
	t := r
	for t > 0 {
		l *= 10
		t /= 10
	}
	return l + r
}

func part1(equations []Equation) int {
	st := stack.NewStack[SolutionState](12)

	solved := lo.Filter(equations, func(eq Equation, _ int) bool {
		return solveEquation(&eq, false, &st)
	})
	return lo.SumBy(solved, func(eq Equation) int { return eq.result })
}

func part2(equations []Equation) int {
	st := stack.NewStack[SolutionState](21)

	solved := lo.Filter(equations, func(eq Equation, _ int) bool {
		return solveEquation(&eq, true, &st)
	})
	return lo.SumBy(solved, func(eq Equation) int { return eq.result })
}
