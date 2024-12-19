package day18

import (
	"adventofcode/astar"
	"adventofcode/grid"
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(2264607, part1(input))
	util.AssertEqual(19457120, part2(input))
}

type Input struct {
	bytes  []grid.Coord
	memory grid.Grid[bool]
}

func parseInput(input string, extra ...int) Input {
	width := 71

	// for tests
	if len(extra) > 0 {
		width = extra[0]
	}

	lines := strings.Split(input, "\n")
	byteCoords := lo.Map(lines, func(s string, _ int) []int {
		return lo.Map(strings.Split(s, ","), func(f string, _ int) int { return util.StringToInt(f) })
	})

	memory := grid.MakeGrid[bool](width, width)

	bytes := lo.Map(byteCoords, func(c []int, _ int) grid.Coord { p, _ := memory.Bounds.Compose(c[0], c[1]); return p })

	return Input{bytes, memory}
}

var directions = grid.Directions()

func minimumSteps(memory *grid.Grid[bool], start grid.Coord, end grid.Coord) int {
	initial := start

	atGoal := func(pos grid.Coord) bool {
		return pos == end
	}

	heuristic := func(pos grid.Coord) int {
		return memory.Bounds.ManhattanDistance(pos, end)
	}

	neighbours := func(pos grid.Coord) []astar.Neighbour[grid.Coord] {
		return lo.FilterMap(directions[:], func(d grid.Direction, _ int) (astar.Neighbour[grid.Coord], bool) {
			n, ok := memory.Neighbour(pos, d)
			if ok && !memory.At(n) {
				// neighbour is in-bounds and not corrupted
				return astar.Neighbour[grid.Coord]{Val: n, Cost: 1}, true
			} else {
				return astar.Neighbour[grid.Coord]{}, false
			}
		})
	}

	score, _, ok := astar.Solve(initial, atGoal, heuristic, neighbours, astar.None)
	if !ok {
		panic("no solution")
	}

	return score
}

func part1(input Input, extra ...int) int {
	fallen := 1024
	memory := input.memory.Clone()

	// for tests
	if len(extra) > 0 {
		fallen = extra[0]
	}

	// set corrupted locations
	for _, p := range input.bytes[0:fallen] {
		memory.Set(p, true)
	}

	width := memory.Bounds.Width
	start, _ := memory.Bounds.Compose(0, 0)
	end, _ := memory.Bounds.Compose(width-1, width-1)
	return minimumSteps(&memory, start, end)
}

func part2(input Input) int {
	return 0
}
