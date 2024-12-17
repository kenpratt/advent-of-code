package day16

import (
	"adventofcode/astar"
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(88416, part1(input))
	util.AssertEqual(0, part2(input))
}

type Input struct {
	maze   grid.Grid[bool] // true = no wall, false = wall
	start  grid.Coord
	end    grid.Coord
	facing grid.Direction
}

func parseInput(input string) Input {
	var start, end grid.Coord

	maze := grid.Parse[bool](input, func(c rune, p grid.Coord) bool {
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

	facing := grid.East
	return Input{maze, start, end, facing}
}

type State struct {
	pos    grid.Coord
	facing grid.Direction
}

func part1(input Input) int {
	initial := State{input.start, input.facing}

	atGoal := func(s State, g int) bool {
		return s.pos == input.end
	}

	heuristic := func(s State) int {
		return input.maze.Bounds.ManhattanDistance(s.pos, input.end)
	}

	neighbours := func(s State) []util.Tuple[State, int] {
		// we can always rotate CW/CCW at cost 1000
		res := []util.Tuple[State, int]{
			util.MakeTuple(State{s.pos, s.facing.Clockwise()}, 1000),
			util.MakeTuple(State{s.pos, s.facing.CounterClockwise()}, 1000),
		}

		// can we move forward?
		if ahead, ok := input.maze.Neighbour(s.pos, s.facing); ok {
			// not a wall?
			if input.maze.At(ahead) {
				// we can move ahead at cost 1
				res = append(res, util.MakeTuple(State{ahead, s.facing}, 1))
			}
		}

		return res
	}

	solution, ok := astar.Solve(initial, atGoal, heuristic, neighbours)
	if !ok {
		panic("no solution")
	}
	return solution
}

func part2(input Input) int {
	return 0
}
