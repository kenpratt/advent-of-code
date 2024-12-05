package day05

import (
	"adventofcode/util"
	"fmt"
	"strings"

	mapset "github.com/deckarep/golang-set/v2"
	"github.com/samber/lo"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	fmt.Println("part 1: ", part1(input))
	fmt.Println("part 2: ", part2(input))
}

type Spec struct {
	orderingRules [][2]int
	toProduce     [][]int
}

func parseInput(input string) Spec {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	orderingRules := parsePageOrderingRules(parts[0])
	toProduce := parsePagesToProduce(parts[1])
	return Spec{orderingRules, toProduce}
}

func parsePageOrderingRules(input string) [][2]int {
	lines := strings.Split(input, "\n")

	rules := make([][2]int, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, "|")
		util.AssertEqual(2, len(parts))

		rules[i][0] = util.StringToInt(parts[0])
		rules[i][1] = util.StringToInt(parts[1])
	}

	return rules
}

func parsePagesToProduce(input string) [][]int {
	lines := strings.Split(input, "\n")
	return lo.Map(lines, func(line string, _ int) []int {
		parts := strings.Split(line, ",")
		return lo.Map(parts, func(s string, _ int) int {
			return util.StringToInt(s)
		})
	})
}

func pagesInCorrectOrder(pages []int, rules mapset.Set[[2]int]) bool {
	// iterate through pages in reverse order, checking pairs
	// if there is a pair in the set of rules, than they must be in the
	// incorrect order
	for i := len(pages) - 1; i >= 0; i-- {
		for j := i - 1; j >= 0; j-- {
			pair := [2]int{pages[i], pages[j]}
			if rules.Contains(pair) {
				return false
			}
		}
	}
	return true
}

func part1(input string) int {
	spec := parseInput(input)

	rules := mapset.NewSet(spec.orderingRules...)

	return lo.SumBy(spec.toProduce, func(pages []int) int {
		if pagesInCorrectOrder(pages, rules) {
			return pages[len(pages)/2]
		} else {
			return 0
		}
	})
}

func part2(input string) int {
	return 0
}
