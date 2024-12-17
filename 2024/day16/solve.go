package day16

import (
	"adventofcode/astar"
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/util"
	"fmt"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(88416, part1(input))
	util.AssertEqual(442, part2(input))
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

func solve(input Input, part2 bool) int {
	initial := State{input.start, input.facing}

	atGoal := func(s State) bool {
		return s.pos == input.end
	}

	heuristic := func(s State) int {
		px, py := input.maze.Bounds.Decompose(s.pos)
		ex, ey := input.maze.Bounds.Decompose(input.end)

		rotations := 0

		if px < ex {
			switch s.facing {
			case grid.North, grid.South:
				rotations++
			case grid.West:
				rotations += 2
			}
		} else if px > ex {
			switch s.facing {
			case grid.North, grid.South:
				rotations++
			case grid.East:
				rotations += 2
			}
		}

		if py < ey {
			switch s.facing {
			case grid.West, grid.East:
				rotations++
			case grid.North:
				rotations += 2
			}
		} else if py > ey {
			switch s.facing {
			case grid.West, grid.East:
				rotations++
			case grid.South:
				rotations += 2
			}
		}

		return input.maze.Bounds.ManhattanDistance(s.pos, input.end) + rotations*1000
	}

	neighbours := func(s State) []astar.Neighbour[State] {
		// we can always rotate CW/CCW at cost 1000
		res := []astar.Neighbour[State]{
			{Val: State{s.pos, s.facing.Clockwise()}, Cost: 1000},
			{Val: State{s.pos, s.facing.CounterClockwise()}, Cost: 1000},
		}

		// can we move forward?
		if ahead, ok := input.maze.Neighbour(s.pos, s.facing); ok {
			// not a wall?
			if input.maze.At(ahead) {
				// we can move ahead at cost 1
				res = append(res, astar.Neighbour[State]{Val: State{ahead, s.facing}, Cost: 1})
			}
		}

		return res
	}

	if !part2 {
		// part 1, just return the cost of the best solution
		score, _, ok := astar.Solve(initial, atGoal, heuristic, neighbours, astar.None)
		if !ok {
			panic("no solution")
		}
		return score
	} else {
		// part 2, find all paths with the best cost, and return the number of
		// unique locations in those paths
		_, paths, ok := astar.Solve(initial, atGoal, heuristic, neighbours, astar.All)
		if !ok {
			panic("no solution")
		}

		locs := set.NewSet[grid.Coord]()
		for _, path := range paths {
			for _, s := range path {
				locs.Add(s.pos)
			}
		}
		return locs.Len()
	}
}

func part1(input Input) int {
	return solve(input, false)
}

func part2(input Input) int {
	return solve(input, true)
}
