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
	util.AssertEqual(841533074412361, part2(input))
}

func parseInput(input string) []int {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	towels := strings.Split(parts[0], ", ")
	designs := strings.Split(parts[1], "\n")

	towelSet := set.NewSet(towels...)
	toTry := pqueue.MakePriorityQueue[int](40)
	return lo.Map(designs, func(design string, _ int) int { return possibleArrangements(design, &towelSet, &toTry) })
}

func possibleArrangements(design string, towels *set.Set[string], toTry *pqueue.PriorityQueue[int]) int {
	designWidth := len(design)
	toTry.Clear()

	tried := make([]bool, designWidth+1)
	connectsTo := make([][]int, designWidth)

	minWidth, maxWidth := 1000, 0
	for t := range towels.Iter() {
		minWidth = min(minWidth, len(t))
		maxWidth = max(maxWidth, len(t))
	}

	// start with index 0
	toTry.Push(0, 0)

	// build up a map of where we can go from each location
	for toTry.Len() > 0 {
		i, _ := toTry.Pop()
		if tried[i] {
			panic("We already tried this one - should be unreachable")
		}

		tried[i] = true
		connectsTo[i] = make([]int, 0)

		for width := minWidth; width <= maxWidth; width++ {
			j := i + width
			if j > designWidth {
				// skip, we don't have enough letters remaining for this towel width
				continue
			}

			if towels.Contains(design[i:j]) {
				connectsTo[i] = append(connectsTo[i], j)

				if j == designWidth {
					// we made it to the end!
				} else {
					// we can progress further
					if !tried[j] {
						toTry.Push(j, -j) // -j because we want the biggest value first
					} else {
						// we already tried this length so there's no point revisiting it
					}
				}
			}
		}
	}

	scoring := make([]int, designWidth+1)
	scoring[0] = 1
	for i := 0; i < designWidth; i++ {
		if tried[i] {
			score := scoring[i]
			for _, j := range connectsTo[i] {
				scoring[j] += score
			}
		}
	}

	return scoring[designWidth]
}

func part1(arrangements []int) int {
	return lo.CountBy(arrangements, func(n int) bool { return n > 0 })
}

func part2(arrangements []int) int {
	return lo.Sum(arrangements)
}
