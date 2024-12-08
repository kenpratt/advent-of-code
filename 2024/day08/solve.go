package day08

import (
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"
	"strings"

	mapset "github.com/deckarep/golang-set/v2"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	fmt.Println("part 1: ", part1(input))
	fmt.Println("part 2: ", part2(input))
}

func parseInput(input string) (grid.Bounds, map[rune][]grid.Coord) {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := grid.Bounds{Width: width, Height: height}

	byFrequency := make(map[rune][]grid.Coord)

	for y, line := range lines {
		for x, c := range line {
			if c != '.' {
				pos := grid.MakeCoord(x, y)
				byFrequency[c] = append(byFrequency[c], pos)
			}
		}
	}

	return bounds, byFrequency
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
			diff := grid.SubtractCoords(a2, a1)

			n1 := grid.SubtractCoords(a1, &diff)
			if grid.InBounds(&bounds, &n1) {
				antinodes.Add(n1)
			}

			n2 := grid.AddCoords(a2, &diff)
			if grid.InBounds(&bounds, &n2) {
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
			diff := grid.SubtractCoords(a2, a1)

			// go back
			n := *a1
			for grid.InBounds(&bounds, &n) {
				antinodes.Add(n)
				n = grid.SubtractCoords(&n, &diff)
			}

			// go forward
			n = *a2
			for grid.InBounds(&bounds, &n) {
				antinodes.Add(n)
				n = grid.AddCoords(&n, &diff)
			}

		})
	}

	return antinodes.Cardinality()
}
