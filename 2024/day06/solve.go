package day06

import (
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	util.AssertEqual(4647, part1(input))
	util.AssertEqual(1723, part2(input))
}

type Guard struct {
	position    grid.Coord
	orientation grid.Direction
}

type State struct {
	guard            Guard
	visited          grid.Grid[Visited]
	extraObstruction grid.Coord
}

type Visited struct {
	status    VisitedStatus
	direction grid.Direction
}

type VisitedStatus int

const (
	Never VisitedStatus = iota + 0
	Once
	Multiple
)

func parseInput(input string) (grid.Grid[bool], Guard) {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := grid.Bounds{Width: width, Height: height}
	values := make([]bool, width*height)

	guard := Guard{}

	for y, line := range lines {
		for x, char := range line {
			pos := grid.MakeCoord(x, y)

			i := bounds.CoordToIndex(&pos)
			switch char {
			case '#':
				values[i] = true
			case '.':
				values[i] = false
			case '^':
				guard.orientation = grid.North
				guard.position = pos
			default:
				panic("Unexpected char in grid")
			}
		}
	}

	terrain := grid.Grid[bool]{Bounds: bounds, Values: values}

	return terrain, guard
}

type Termination int

const (
	OutOfBounds Termination = iota + 1
	Looping
)

func run(terrain *grid.Grid[bool], state *State) Termination {
	for {
		result, isDone := tick(terrain, state)
		if isDone {
			return result
		}
	}
}

func tick(terrain *grid.Grid[bool], state *State) (Termination, bool) {
	tryMoveTo := ahead(state)
	obstruction, inBounds := terrain.At(&tryMoveTo)
	if !inBounds {
		return OutOfBounds, true
	}

	if *obstruction || state.extraObstruction == tryMoveTo {
		state.guard.orientation = state.guard.orientation.Clockwise()
	} else {
		// no obstruction, move to new location
		visited, _ := state.visited.At(&tryMoveTo)

		if visited.status == Once && visited.direction == state.guard.orientation {
			// whoops, we've already been to this location moving in this direction
			return Looping, true
		}

		// update visited status
		switch visited.status {
		case Never:
			visited.status = Once
			visited.direction = state.guard.orientation
		case Once:
			visited.status = Multiple
		case Multiple:
			// noop
		default:
			panic(fmt.Sprintf("Unknown visited status: %v", visited.status))
		}

		// move to new location
		state.guard.position = tryMoveTo
	}

	return 0, false
}

func ahead(state *State) grid.Coord {
	return state.guard.position.MoveInDirection(state.guard.orientation, 1)
}

func initialState(terrain *grid.Grid[bool], guard Guard) State {
	visited := grid.Grid[Visited]{
		Bounds: terrain.Bounds,
		Values: make([]Visited, len(terrain.Values)),
	}
	visited.Set(&guard.position, Visited{status: Once, direction: guard.orientation})

	return State{
		guard:            guard,
		visited:          visited,
		extraObstruction: grid.MakeCoord(-1, -1),
	}
}

func part1(input string) int {
	terrain, guard := parseInput(input)
	state := initialState(&terrain, guard)

	result := run(&terrain, &state)
	util.AssertEqual(OutOfBounds, result)

	return lo.CountBy[Visited](state.visited.Values, func(visited Visited) bool { return visited.status != Never })
}

func part2(input string) int {
	terrain, guard := parseInput(input)

	loops := 0

	mainState := initialState(&terrain, guard)
	altState := initialState(&terrain, guard)

	for {
		aheadPos := ahead(&mainState)

		// is the ahead pos empty?
		obstruction, inBounds := terrain.At(&aheadPos)
		if inBounds && !*obstruction {
			// check if we've already visited this location
			visited, _ := mainState.visited.At(&aheadPos)
			if !(visited.status == Once || visited.status == Multiple) {
				// try running an alternate simulation with an obstruction here

				// update altState
				altState.guard = mainState.guard
				copy(altState.visited.Values, mainState.visited.Values)
				altState.extraObstruction = aheadPos

				// run and see if the result is a loop
				res := run(&terrain, &altState)
				if res == Looping {
					loops++
				}
			}
		}

		_, isDone := tick(&terrain, &mainState)
		if isDone {
			break
		}
	}

	return loops
}
