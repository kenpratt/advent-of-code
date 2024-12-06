package day06

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

type Guard struct {
	position    grid.Coord
	orientation grid.Direction
	visited     mapset.Set[grid.Coord]
}

type State struct {
	guard        Guard
	obstructions grid.Grid[bool]
}

func parseInput(input string) State {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := grid.Bounds{Width: width, Height: height}
	values := make([]bool, width*height)

	guard := Guard{
		orientation: grid.North,
		visited:     mapset.NewSet[grid.Coord](),
	}

	for y, line := range lines {
		for x, char := range line {
			pos := grid.MakeCoord(x, y)

			values[grid.CoordToIndex(bounds, pos)] = char == '#'

			if char == '^' {
				guard.position = pos
				guard.visited.Add(pos)
			}
		}
	}

	obstructions := grid.Grid[bool]{Bounds: bounds, Values: values}

	return State{guard, obstructions}
}

func run(state *State) {
	for {
		tryMoveTo := grid.MoveInDirection(state.guard.position, state.guard.orientation, 1)
		obstruction, inBounds := grid.At(state.obstructions, tryMoveTo)
		if !inBounds {
			return
		}

		if obstruction {
			state.guard.orientation = grid.TurnRight(state.guard.orientation)
		} else {
			state.guard.position = tryMoveTo
			state.guard.visited.Add(tryMoveTo)
		}
	}
}

func part1(input string) int {
	state := parseInput(input)
	run(&state)
	return state.guard.visited.Cardinality()
}

func part2(input string) int {
	return 0
}
