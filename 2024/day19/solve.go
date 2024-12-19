package day19

import (
	"adventofcode/pqueue"
	"adventofcode/set"
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(338, part1(input))
	util.AssertEqual(0, part2(input))
}

type Input struct {
	towels  []string
	designs []string
}

func parseInput(input string) Input {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	towels := strings.Split(parts[0], ", ")
	designs := strings.Split(parts[1], "\n")
	return Input{towels, designs}
}

func isPossible(design string, towels *set.Set[string]) bool {
	designWidth := len(design)

	tried := make([]bool, designWidth+1)
	toTry := pqueue.MakePriorityQueue[int]()

	minWidth, maxWidth := 1000, 0
	for t := range towels.Iter() {
		minWidth = min(minWidth, len(t))
		maxWidth = max(maxWidth, len(t))
	}

	// start with index 0
	toTry.Push(0, 0)

	// keep going until we either find a solution, or we run out of options
	for toTry.Len() > 0 {
		i, _ := toTry.Pop()
		if tried[i] {
			panic("We already tried this one - should be unreachable")
		}

		tried[i] = true

		for width := minWidth; width <= maxWidth; width++ {
			j := i + width
			if j > designWidth {
				// skip, we don't have enough letters remaining for this towel width
				continue
			}

			if tried[j] {
				// we already tried this length so there's no point even checking if it works
				continue
			}

			if towels.Contains(design[i:j]) {
				if j == designWidth {
					// we made it to the end!
					return true
				} else {
					// we can progress further
					toTry.Push(j, -j) // -j because we want the biggest value first
				}
			}
		}
	}

	return false
}

func part1(input Input) int {
	towels := set.NewSet(input.towels...)
	return lo.CountBy(input.designs, func(design string) bool { return isPossible(design, &towels) })
}

func part2(input Input) int {
	return 0
}
