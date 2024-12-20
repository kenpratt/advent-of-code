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
	util.AssertEqual(1032257, part2(input))
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

func solve(input *Input, maxCheatLength int, extra ...int) int {
	saveAtLeast := 100
	if len(extra) > 0 {
		saveAtLeast = extra[0]
	}

	mainPath := nonCheatingPath(input)

	// grid used as lookup table for pos -> path index
	pathIndices := grid.MakeGrid[int](input.track.Bounds.Width, input.track.Bounds.Height)
	for i, pos := range mainPath {
		pathIndices.Set(pos, i)
	}

	// catalog the cheats
	cheats := make([]int, len(mainPath)-1)
	for i := 0; i < len(mainPath)-1; i++ {
		// for each pos, find nearby spots later along the path
		pos := mainPath[i]
		px, py := pathIndices.Bounds.Decompose(pos)

		// x search range
		fromX := max(px-maxCheatLength, 0)
		toX := min(px+maxCheatLength, pathIndices.Bounds.Width-1)
		for cx := fromX; cx <= toX; cx++ {
			// clamp into diamond shape (the further we are from px, the less we need to explore y)
			dx := util.AbsDiff(px, cx)
			dy := maxCheatLength - dx

			// y search range
			fromY := max(py-dy, 0)
			toY := min(py+dy, pathIndices.Bounds.Height-1)
			for cy := fromY; cy <= toY; cy++ {
				// this is a bit faster than bounds.Compose(cx, cy)
				c := grid.Coord(int(pos) + (cx - px) + (cy-py)*pathIndices.Bounds.Width)

				// is j further ahead in the path? not behind?
				j := pathIndices.At(c)
				if j > i {
					// okay cool, then it's a shortcut
					shortcut := util.AbsDiff(px, cx) + util.AbsDiff(py, cy)
					usual := j - i
					saved := usual - shortcut
					if saved > 0 {
						cheats[saved]++
					}
				}
			}
		}
	}

	return lo.Sum(cheats[saveAtLeast:])
}

func part1(input Input, extra ...int) int {
	return solve(&input, 2, extra...)
}

func part2(input Input, extra ...int) int {
	return solve(&input, 20, extra...)
}
