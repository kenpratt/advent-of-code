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
	index int
	acc   int
}

func solveEquation(equation *Equation, enableConcatenation bool, st *stack.Stack[SolutionState]) bool {
	initialState := SolutionState{
		index: 1,
		acc:   equation.values[0],
	}
	st.Push(initialState)

	for st.Len() != 0 {
		state := st.Pop()
		i := state.index
		acc := state.acc
		j := i + 1
		val := equation.values[i]

		// try add
		added := acc + val
		switch evaluate(j, added, equation) {
		case Solution:
			st.Clear()
			return true
		case Possible:
			st.Push(SolutionState{index: j, acc: added})
		}

		// try multiply
		multiplied := acc * val
		switch evaluate(j, multiplied, equation) {
		case Solution:
			st.Clear()
			return true
		case Possible:
			st.Push(SolutionState{index: j, acc: multiplied})
		}

		if enableConcatenation {
			// try concatenation
			concatenated := util.ConcatenateInts(acc, val)
			switch evaluate(j, concatenated, equation) {
			case Solution:
				st.Clear()
				return true
			case Possible:
				st.Push(SolutionState{index: j, acc: concatenated})
			}
		}
	}

	// no solution found
	return false
}

type Evaluation int

const (
	Solution Evaluation = iota
	Possible
	Impossible
)

func evaluate(i int, acc int, eq *Equation) Evaluation {
	if i == len(eq.values) {
		// we're at the end
		if acc == eq.result {
			return Solution
		} else {
			return Impossible
		}
	} else {
		// still have more to explore
		if acc <= eq.result {
			return Possible
		} else {
			// acc is bigger than the result, no point in continuing
			return Impossible
		}
	}
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
