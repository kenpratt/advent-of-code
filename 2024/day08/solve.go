package day08

import (
	"adventofcode/grid"
	"adventofcode/util"
	"strings"

	mapset "github.com/deckarep/golang-set/v2"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	util.AssertEqual(394, part1(input))
	util.AssertEqual(1277, part2(input))
}

func parseInput(input string) (bounds grid.Bounds, byFrequency map[rune][]grid.Coord) {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds = grid.Bounds{Width: width, Height: height}

	byFrequency = make(map[rune][]grid.Coord)

	for y, line := range lines {
		for x, c := range line {
			if c != '.' {
				pos := grid.MakeCoord(x, y)
				byFrequency[c] = append(byFrequency[c], pos)
			}
		}
	}

	return
}

func iterPairs[T any](slice []T, fn func(*T, *T)) {
	for i := 0; i < len(slice); i++ {
		for j := i + 1; j < len(slice); j++ {
			fn(&slice[i], &slice[j])
		}
	}
}

func part1(input string) int {
	bounds, byFrequency := parseInput(input)

	antinodes := mapset.NewSet[grid.Coord]()

	for _, antennas := range byFrequency {
		iterPairs(antennas, func(a1 *grid.Coord, a2 *grid.Coord) {
			diff := a2.Subtract(a1)

			n1 := a1.Subtract(&diff)
			if bounds.Within(&n1) {
				antinodes.Add(n1)
			}

			n2 := a2.Add(&diff)
			if bounds.Within(&n2) {
				antinodes.Add(n2)
			}
		})
	}

	return antinodes.Cardinality()
}

func part2(input string) int {
	bounds, byFrequency := parseInput(input)

	antinodes := mapset.NewSet[grid.Coord]()

	for _, antennas := range byFrequency {
		iterPairs(antennas, func(a1 *grid.Coord, a2 *grid.Coord) {
			diff := a2.Subtract(a1)

			// go back
			n := *a1
			for bounds.Within(&n) {
				antinodes.Add(n)
				n = n.Subtract(&diff)
			}

			// go forward
			n = *a2
			for bounds.Within(&n) {
				antinodes.Add(n)
				n = n.Add(&diff)
			}

		})
	}

	return antinodes.Cardinality()
}
