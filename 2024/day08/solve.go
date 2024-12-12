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
			diff := a2.Subtract(a1)

			n1 := a1.Subtract(diff)
			if bounds.Within(n1) {
				antinodes.Add(n1)
			}

			n2 := a2.Add(diff)
			if bounds.Within(n2) {
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
			diff := a2.Subtract(a1)

			// go back
			n := a1
			for bounds.Within(n) {
				antinodes.Add(n)
				n = n.Subtract(diff)
			}

			// go forward
			n = a2
			for bounds.Within(n) {
				antinodes.Add(n)
				n = n.Add(diff)
			}
		})
	}

	return antinodes.Len()
}
