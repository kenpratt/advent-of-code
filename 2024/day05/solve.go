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
	orderingRules mapset.Set[[2]int]
	toProduce     [][]int
}

func parseInput(input string) Spec {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	orderingRules := parsePageOrderingRules(parts[0])
	toProduce := parsePagesToProduce(parts[1])
	return Spec{orderingRules, toProduce}
}

func parsePageOrderingRules(input string) mapset.Set[[2]int] {
	lines := strings.Split(input, "\n")

	rules := mapset.NewSet[[2]int]()

	for _, line := range lines {
		parts := strings.Split(line, "|")
		util.AssertEqual(2, len(parts))
		rules.Add([2]int{util.StringToInt(parts[0]), util.StringToInt(parts[1])})
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

func arePagesInCorrectOrder(pages []int, rules mapset.Set[[2]int]) bool {
	// iterate through pages in reverse order, checking pairs
	// if there is a pair in the set of rules, than they must be in the
	// incorrect order
	for i := len(pages) - 1; i > 0; i-- {
		pair := [2]int{pages[i], pages[i-1]}
		if rules.Contains(pair) {
			return false
		}
	}
	return true
}

func findCorrectMiddlePage(pages []int, rules mapset.Set[[2]int]) int {
	pageSet := mapset.NewSet(pages...)
	target := len(pages) / 2
	for _, page := range pages {
		before := 0
		after := 0
		for rule := range rules.Iter() {
			if rule[0] == page && pageSet.Contains(rule[1]) {
				before++
				if before > target {
					break // can quit, too many element before middle
				}
			} else if rule[1] == page && pageSet.Contains(rule[0]) {
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

func part1(input string) int {
	spec := parseInput(input)

	return lo.SumBy(spec.toProduce, func(pages []int) int {
		if arePagesInCorrectOrder(pages, spec.orderingRules) {
			return pages[len(pages)/2]
		} else {
			return 0
		}
	})
}

func part2(input string) int {
	spec := parseInput(input)

	return lo.SumBy(spec.toProduce, func(pages []int) int {
		if arePagesInCorrectOrder(pages, spec.orderingRules) {
			return 0
		} else {
			return findCorrectMiddlePage(pages, spec.orderingRules)
		}
	})
}
