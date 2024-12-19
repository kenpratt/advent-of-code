package day18

import (
	"adventofcode/astar"
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(272, part1(input))
	util.AssertEqual("16,44", part2(input))
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

type MinimumSteps struct {
	memory *grid.Grid[bool]
	start  grid.Coord
	end    grid.Coord
}

func (s *MinimumSteps) AtGoal(pos grid.Coord) bool {
	return pos == s.end
}

func (s *MinimumSteps) Heuristic(pos grid.Coord) int {
	return s.memory.Bounds.ManhattanDistance(pos, s.end)
}

func (s *MinimumSteps) Neighbours(pos grid.Coord) []astar.Neighbour[grid.Coord] {
	return lo.FilterMap(directions[:], func(d grid.Direction, _ int) (astar.Neighbour[grid.Coord], bool) {
		n, ok := s.memory.Neighbour(pos, d)
		if ok && !s.memory.At(n) {
			// neighbour is in-bounds and not corrupted
			return astar.Neighbour[grid.Coord]{Val: n, Cost: 1}, true
		} else {
			return astar.Neighbour[grid.Coord]{}, false
		}
	})
}

func (s *MinimumSteps) Run() (int, bool) {
	score, _, ok := astar.Solve(s.start, s, astar.None)
	return score, ok
}

func minimumStepsAfterNFallen(input *Input, fallen int) (int, bool) {
	memory := &input.memory
	memory.Clear()

	// set corrupted locations
	for _, p := range input.bytes[0:fallen] {
		memory.Set(p, true)
	}

	width := memory.Bounds.Width
	start, _ := memory.Bounds.Compose(0, 0)
	end, _ := memory.Bounds.Compose(width-1, width-1)

	solver := MinimumSteps{memory, start, end}
	return solver.Run()
}

func part1(input Input, extra ...int) int {
	fallen := 1024

	// for tests
	if len(extra) > 0 {
		fallen = extra[0]
	}

	steps, ok := minimumStepsAfterNFallen(&input, fallen)
	util.AssertEqual(true, ok)
	return steps
}

func part2(input Input) string {
	// binary search to find the first number of bits fallen with no exit
	l := 0
	r := len(input.bytes)
	for {
		m := (l + r) / 2

		_, ok := minimumStepsAfterNFallen(&input, m)
		if ok {
			l = m + 1
		} else {
			r = m - 1
		}

		if l == r {
			p := input.bytes[l-1]
			x, y := input.memory.Bounds.Decompose(p)
			return fmt.Sprintf("%d,%d", x, y)
		}
	}
}
