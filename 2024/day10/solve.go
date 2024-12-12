package day10

import (
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/util"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(822, part1(input))
	util.AssertEqual(1801, part2(input))
}

func parseInput(input string) grid.Grid[int] {
	return grid.Parse(input, func(c rune, _ grid.Coord) int {
		return util.RuneToInt(c)
	})
}

func part1(topo grid.Grid[int]) int {
	directions := grid.Directions()

	// lookup by height
	heightIndices := make(map[int][]int)
	for i, height := range topo.Values {
		heightIndices[height] = append(heightIndices[height], i)
	}

	score := 0

	// follow each trailhead
	for _, trailhead := range heightIndices[0] {
		openSet := set.NewSet[int](trailhead)

		// for each height, find the unique positions it leads to
		for height := 1; height < 10; height++ {
			nextOpenSet := set.NewSet[int]()
			for i := range openSet.Iter() {
				for _, d := range directions {
					n, inBounds := topo.NeighbourForIndex(i, d)
					if inBounds && topo.Values[n] == height {
						nextOpenSet.Add(n)
					}
				}
			}
			openSet = nextOpenSet
		}

		score += openSet.Len()
	}

	return score
}

func part2(topo grid.Grid[int]) int {
	directions := grid.Directions()

	// lookup by height
	heightIndices := make(map[int][]int)
	for i, height := range topo.Values {
		heightIndices[height] = append(heightIndices[height], i)
	}

	// scores per location
	scores := make([]int, topo.Len())

	// pre-seed scores of height 9
	for _, i := range heightIndices[9] {
		scores[i] = 1
	}

	// now go from 8 down to 0, calculating scores
	for height := 8; height >= 0; height-- {
		for _, i := range heightIndices[height] {
			for _, d := range directions {
				n, inBounds := topo.NeighbourForIndex(i, d)
				if inBounds && topo.Values[n] == height+1 {
					scores[i] += scores[n]
				}
			}
		}
	}

	// add up height 0 scores
	result := 0
	for _, i := range heightIndices[0] {
		result += scores[i]
	}
	return result
}
