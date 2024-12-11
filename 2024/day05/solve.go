package day05

import (
	"adventofcode/set"
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(6267, part1(input))
	util.AssertEqual(5184, part2(input))
}

type Spec struct {
	orderingRules set.Set[Rule]
	toProduce     [][]int
}

type Rule struct {
	left  int
	right int
}

func parseInput(input string) Spec {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	orderingRules := parsePageOrderingRules(parts[0])
	toProduce := parsePagesToProduce(parts[1])
	return Spec{orderingRules, toProduce}
}

func parsePageOrderingRules(input string) set.Set[Rule] {
	lines := strings.Split(input, "\n")

	rules := set.NewSet[Rule]()

	for _, line := range lines {
		parts := strings.Split(line, "|")
		util.AssertEqual(2, len(parts))
		rules.Add(Rule{util.StringToInt(parts[0]), util.StringToInt(parts[1])})
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

func arePagesInCorrectOrder(pages []int, rules set.Set[Rule]) bool {
	// iterate through pages in reverse order, checking pairs
	// if there is a pair in the set of rules, than they must be in the
	// incorrect order
	for i := len(pages) - 1; i > 0; i-- {
		pair := Rule{pages[i], pages[i-1]}
		if rules.Contains(pair) {
			return false
		}
	}
	return true
}

func findCorrectMiddlePage(pages []int, rules set.Set[Rule]) int {
	pageSet := set.NewSet(pages...)
	target := len(pages) / 2
	for _, page := range pages {
		before := 0
		after := 0
		for rule := range rules.Iter() {
			if rule.left == page && pageSet.Contains(rule.right) {
				before++
				if before > target {
					break // can quit, too many element before middle
				}
			} else if rule.right == page && pageSet.Contains(rule.left) {
				after++
				if after > target {
					break // can quit, too many elements after middle
				}
			}
		}
		if before == target && after == target {
			// we know this is the middle page because there are exactly N elements before and after it
			return page
		}
	}
	panic("Didn't find the correct middle page")
}

func part1(spec Spec) int {
	return lo.SumBy(spec.toProduce, func(pages []int) int {
		if arePagesInCorrectOrder(pages, spec.orderingRules) {
			return pages[len(pages)/2]
		} else {
			return 0
		}
	})
}

func part2(spec Spec) int {
	return lo.SumBy(spec.toProduce, func(pages []int) int {
		if arePagesInCorrectOrder(pages, spec.orderingRules) {
			return 0
		} else {
			return findCorrectMiddlePage(pages, spec.orderingRules)
		}
	})
}
