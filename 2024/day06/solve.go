package day06

import (
	"adventofcode/grid"
	"adventofcode/util"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(4647, part1(input))
	util.AssertEqual(1723, part2(input))
}

type Guard struct {
	position    grid.Coord
	orientation grid.Direction
}

type State struct {
	guard            Guard
	visited          grid.Grid[uint8]
	extraObstruction grid.Coord
}

type Input struct {
	terrain grid.Grid[bool]
	guard   Guard
}

func parseInput(input string) Input {
	guard := Guard{}
	terrain := grid.Parse(input, func(c rune, pos grid.Coord) bool {
		switch c {
		case '#':
			return true
		case '.':
			return false
		case '^':
			guard.orientation = grid.North
			guard.position = pos
			return false
		default:
			panic("Unexpected char in grid")
		}
	})

	return Input{terrain, guard}
}

type Termination uint8

const (
	OutOfBounds Termination = iota + 1
	Looping
)

func run(terrain *grid.Grid[bool], state *State, cache *grid.NeighbourCache) Termination {
	for {
		result, isDone := tick(terrain, state, cache)
		if isDone {
			return result
		}
	}
}

func tick(terrain *grid.Grid[bool], state *State, cache *grid.NeighbourCache) (Termination, bool) {
	tryMoveTo, inBounds := ahead(state, cache)
	if !inBounds {
		return OutOfBounds, true
	}

	obstruction := terrain.At(tryMoveTo)
	if obstruction || state.extraObstruction == tryMoveTo {
		state.guard.orientation = state.guard.orientation.Clockwise()
	} else {
		// no obstruction, move to new location
		visited := state.visited.AtMut(tryMoveTo)

		switch *visited {
		case 0:
			// special case - never visited
			// now we've visited once, store the direction
			*visited = uint8(state.guard.orientation)
		case 5:
			// special case - multiple visits
			// we've already been here multiple times - noop
		case uint8(state.guard.orientation):
			// second visit, in the same orientation
			// whoops, we've already been to this location moving in this direction
			return Looping, true
		default:
			// second visit, different orientation
			*visited = 5 // special case, 5 represents multiple visits

		}

		// move to new location
		state.guard.position = tryMoveTo
	}

	return 0, false
}

func ahead(state *State, cache *grid.NeighbourCache) (grid.Coord, bool) {
	return cache.Neighbour(state.guard.position, state.guard.orientation)
}

func initialState(terrain *grid.Grid[bool], guard Guard) State {
	visited := grid.Grid[uint8]{
		Bounds: terrain.Bounds,
		Values: make([]uint8, len(terrain.Values)),
	}
	visited.Set(guard.position, uint8(guard.orientation))

	return State{
		guard:            guard,
		visited:          visited,
		extraObstruction: grid.Coord(-1),
	}
}

func part1(input Input) int {
	terrain, guard := input.terrain, input.guard
	state := initialState(&terrain, guard)
	neighbourCache := grid.MakeNeighbourCache(terrain.Bounds)

	result := run(&terrain, &state, &neighbourCache)
	util.AssertEqual(OutOfBounds, result)

	return lo.CountBy(state.visited.Values, func(visited uint8) bool { return visited > 0 })
}

func part2(input Input) int {
	terrain, guard := input.terrain, input.guard

	loops := 0

	mainState := initialState(&terrain, guard)
	altState := initialState(&terrain, guard)
	neighbourCache := grid.MakeNeighbourCache(terrain.Bounds)

	for {
		aheadPos, inBounds := ahead(&mainState, &neighbourCache)

		// is the ahead pos empty?
		if inBounds && !terrain.At(aheadPos) {
			// check if we've already visited this location
			visited := mainState.visited.At(aheadPos)
			if visited == 0 {
				// try running an alternate simulation with an obstruction here

				// update altState
				altState.guard = mainState.guard
				copy(altState.visited.Values, mainState.visited.Values)
				altState.extraObstruction = aheadPos

				// run and see if the result is a loop
				res := run(&terrain, &altState, &neighbourCache)
				if res == Looping {
					loops++
				}
			}
		}

		_, isDone := tick(&terrain, &mainState, &neighbourCache)
		if isDone {
			break
		}
	}

	return loops
}
