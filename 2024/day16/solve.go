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
	solution := parseInput(inputStr)
	util.AssertEqual(88416, part1(solution))
	util.AssertEqual(442, part2(solution))
}

type Input struct {
	maze   grid.Grid[bool] // true = no wall, false = wall
	start  grid.Coord
	end    grid.Coord
	facing grid.Direction
}

func parseInput(s string) Solution {
	var start, end grid.Coord

	maze := grid.Parse[bool](s, func(c rune, p grid.Coord) bool {
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
	input := Input{maze, start, end, facing}
	return solve(&input)
}

type State struct {
	pos    grid.Coord
	facing grid.Direction
}

type Solution struct {
	score int
	paths [][]State
}

type Solver struct {
	input *Input
}

func (r *Solver) AtGoal(s State) bool {
	return s.pos == r.input.end
}

func (r *Solver) Heuristic(s State) int {
	px, py := r.input.maze.Bounds.Decompose(s.pos)
	ex, ey := r.input.maze.Bounds.Decompose(r.input.end)

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

	return r.input.maze.Bounds.ManhattanDistance(s.pos, r.input.end) + rotations*1000
}

func (r *Solver) Neighbours(s State) []astar.Neighbour[State] {
	// we can always rotate CW/CCW at cost 1000
	res := []astar.Neighbour[State]{
		{Val: State{s.pos, s.facing.Clockwise()}, Cost: 1000},
		{Val: State{s.pos, s.facing.CounterClockwise()}, Cost: 1000},
	}

	// can we move forward?
	if ahead, ok := r.input.maze.Neighbour(s.pos, s.facing); ok {
		// not a wall?
		if r.input.maze.At(ahead) {
			// we can move ahead at cost 1
			res = append(res, astar.Neighbour[State]{Val: State{ahead, s.facing}, Cost: 1})
		}
	}

	return res
}

func (r *Solver) Run() Solution {
	initial := State{r.input.start, r.input.facing}
	score, paths, ok := astar.Solve(initial, r, astar.All)
	if !ok {
		panic("no solution")
	}
	return Solution{score, paths}
}

func solve(input *Input) Solution {
	r := Solver{input}
	return r.Run()
}

func part1(solution Solution) int {
	// just return the cost of the best solution
	return solution.score
}

func part2(solution Solution) int {
	// find all paths with the best cost, and return the number of
	// unique locations in those paths
	locs := set.NewSet[grid.Coord]()
	for _, path := range solution.paths {
		for _, s := range path {
			locs.Add(s.pos)
		}
	}
	return locs.Len()
}
