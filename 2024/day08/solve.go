package day08

import (
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/util"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(394, part1(input))
	util.AssertEqual(1277, part2(input))
}

type Input struct {
	bounds      grid.Bounds
	byFrequency map[rune][]grid.Coord
}

func parseInput(input string) Input {
	byFrequency := make(map[rune][]grid.Coord)
	bounds := grid.ParseBoundsAndCoords(input, func(c rune, pos grid.Coord) {
		if c != '.' {
			byFrequency[c] = append(byFrequency[c], pos)
		}
	})
	return Input{bounds, byFrequency}
}

func iterPairs[T any](slice []T, fn func(T, T)) {
	for i := 0; i < len(slice); i++ {
		for j := i + 1; j < len(slice); j++ {
			fn(slice[i], slice[j])
		}
	}
}

func part1(input Input) int {
	bounds, byFrequency := input.bounds, input.byFrequency

	antinodes := set.NewSet[grid.Coord]()

	for _, antennas := range byFrequency {
		iterPairs(antennas, func(a1 grid.Coord, a2 grid.Coord) {
			diff := bounds.Subtract(a2, a1)

			if n1, ok := bounds.SubtractOffset(a1, diff); ok {
				antinodes.Add(n1)
			}

			if n2, ok := bounds.AddOffset(a2, diff); ok {
				antinodes.Add(n2)
			}
		})
	}

	return antinodes.Len()
}

func part2(input Input) int {
	bounds, byFrequency := input.bounds, input.byFrequency

	antinodes := set.NewSet[grid.Coord]()

	for _, antennas := range byFrequency {
		iterPairs(antennas, func(a1 grid.Coord, a2 grid.Coord) {
			diff := bounds.Subtract(a2, a1)

			// go back
			n := a1
			for {
				antinodes.Add(n)
				v, ok := bounds.SubtractOffset(n, diff)
				if ok {
					n = v
				} else {
					break
				}
			}

			// go forward
			n = a2
			for {
				antinodes.Add(n)
				v, ok := bounds.AddOffset(n, diff)
				if ok {
					n = v
				} else {
					break
				}
			}
		})
	}

	return antinodes.Len()
}
