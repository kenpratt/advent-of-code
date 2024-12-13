package day13

import (
	"adventofcode/util"
	"regexp"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(30413, part1(input))
	util.AssertEqual(92827349540204, part2(input))
}

type Spec struct {
	a     Pair[int]
	b     Pair[int]
	prize Pair[int]
}

type Pair[T any] struct {
	first, second T
}

type Equation struct {
	a, b, r int
}

func parseInput(input string) []Pair[Equation] {
	re := regexp.MustCompile(`\AButton A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)\z`)

	parts := strings.Split(input, "\n\n")
	return lo.Map(parts, func(s string, _ int) Pair[Equation] {
		spec := parseSpec(s, re)
		return spec.toEquation()
	})
}

func parseSpec(input string, re *regexp.Regexp) Spec {
	match := re.FindStringSubmatch(input)
	vals := lo.Map(match[1:], func(s string, _ int) int { return util.StringToInt(s) })
	util.AssertEqual(6, len(vals))
	return Spec{
		a:     Pair[int]{vals[0], vals[1]},
		b:     Pair[int]{vals[2], vals[3]},
		prize: Pair[int]{vals[4], vals[5]},
	}
}

func (s Spec) toEquation() Pair[Equation] {
	return Pair[Equation]{
		Equation{s.a.first, s.b.first, s.prize.first},
		Equation{s.a.second, s.b.second, s.prize.second},
	}
}

func (eq Equation) scale(f int) Equation {
	return Equation{eq.a * f, eq.b * f, eq.r * f}
}

func (eq Equation) subtract(o Equation) Equation {
	return Equation{eq.a - o.a, eq.b - o.b, eq.r - o.r}
}

func solve(eq Pair[Equation]) (Pair[int], bool) {
	e1 := eq.first.scale(eq.second.a)
	e2 := eq.second.scale(eq.first.a)
	d := e1.subtract(e2)
	util.AssertEqual(0, d.a)

	if d.r%d.b == 0 {
		// has n integer solution for b
		b := d.r / d.b

		// solve the first equation for a
		r := eq.first.r - eq.first.b*b

		if r%eq.first.a == 0 {
			// has an integer solution for a as well
			a := r / eq.first.a

			return Pair[int]{a, b}, true
		}
	}

	return Pair[int]{}, false
}

func solutionCost(eq Pair[Equation]) int {
	solution, ok := solve(eq)
	if ok {
		// cost is 3*A + 1*B
		return solution.first*3 + solution.second
	} else {
		return 0
	}
}

func totalSolutionCost(eqs []Pair[Equation]) int {
	return lo.SumBy(eqs, func(eq Pair[Equation]) int {
		return solutionCost(eq)
	})
}

func part1(eqs []Pair[Equation]) int {
	return totalSolutionCost(eqs)
}

func part2(eqs []Pair[Equation]) int {
	for i := range eqs {
		eqs[i].first.r += 10000000000000
		eqs[i].second.r += 10000000000000
	}
	return totalSolutionCost(eqs)
}
