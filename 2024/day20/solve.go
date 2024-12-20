package day20

import (
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1518, part1(input))
	util.AssertEqual(0, part2(input))
}

type Input struct {
	track grid.Grid[bool] // true = no wall, false = wall
	start grid.Coord
	end   grid.Coord
}

func parseInput(s string) Input {
	var start, end grid.Coord

	track := grid.Parse[bool](s, func(c rune, p grid.Coord) bool {
		switch c {
		case '.':
			return true
		case '#':
			return false
		case 'S':
			start = p
			return true
		case 'E':
			end = p
			return true
		default:
			fmt.Printf("unexpected %c", c)
			panic("Unreachable")
		}
	})

	input := Input{track, start, end}
	return input
}

var directions = grid.Directions()

func nonCheatingPath(input *Input) []grid.Coord {
	path := []grid.Coord{input.start}
	last := input.start
	curr := input.start
	for curr != input.end {
		options := lo.FilterMap(directions[:], func(d grid.Direction, _ int) (grid.Coord, bool) {
			n, ok := input.track.Neighbour(curr, d)
			if ok && input.track.At(n) && n != last {
				// neighbour is in-bounds and not a wall
				return n, true
			} else {
				return grid.Coord(-1), false
			}
		})
		util.AssertEqual(1, len(options)) // there should only be one option
		ahead := options[0]

		path = append(path, ahead)
		last = curr
		curr = ahead
	}
	return path
}

func part1(input Input, extra ...int) int {
	saveAtLeast := 100
	if len(extra) > 0 {
		saveAtLeast = extra[0]
	}

	mainPath := nonCheatingPath(&input)

	// catalog the cheats
	cheats := make([]int, len(mainPath)-1)
	for i := 0; i < len(mainPath)-1; i++ {
		for j := i + 1; j < len(mainPath); j++ {
			// how much time would we need for this cheat?
			shortcut := input.track.Bounds.ManhattanDistance(mainPath[i], mainPath[j])
			if shortcut <= 2 {
				usual := j - i
				saved := usual - shortcut
				if saved > 0 {
					cheats[saved]++
				}
			}
		}
	}

	return lo.Sum(cheats[saveAtLeast:])
}

func part2(input Input) int {
	return 0
}
