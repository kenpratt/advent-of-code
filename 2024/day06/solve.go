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
	terrain          grid.Grid[bool]
	guard            Guard
	neighbourCache   grid.NeighbourCache
	obstructionCache ObstructionCache
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

	neighbourCache := grid.MakeNeighbourCache(terrain.Bounds)
	obstructionCache := MakeObstructionCache(&terrain, &neighbourCache)

	return Input{terrain, guard, neighbourCache, obstructionCache}
}

type Termination uint8

const (
	OutOfBounds Termination = iota + 1
	Looping
)

func run(terrain *grid.Grid[bool], state *State, obstructionCache *ObstructionCache) Termination {
	for {
		result, isDone := tick(terrain, state, obstructionCache)
		if isDone {
			return result
		}
	}
}

func tick(terrain *grid.Grid[bool], state *State, obstructionCache *ObstructionCache) (Termination, bool) {
	// move to just before the next obstruction
	beforeObstruction, isObstructed := state.NextObstruction(terrain, obstructionCache)

	// mark the path along the way as visited
	status, terminated := state.MarkPathVisited(state.guard.position, beforeObstruction, state.guard.orientation, terrain)
	if terminated {
		return status, true
	}

	if !isObstructed {
		return OutOfBounds, true
	} else {
		// move guard to the new location and rotate
		if beforeObstruction == state.extraObstruction {
			panic("trying to move guard on top of extra obstruction")
		}
		state.guard.position = beforeObstruction
		state.guard.orientation = state.guard.orientation.Clockwise()
		return 0, false
	}
}

func (s *State) MarkPathVisited(from grid.Coord, to grid.Coord, d grid.Direction, terrain *grid.Grid[bool]) (Termination, bool) {
	ok := terrain.ForLinearPath(from, to, func(pos grid.Coord) bool {
		visited := s.visited.AtMut(pos)
		switch *visited {
		case 0:
			// special case - never visited
			// now we've visited once, store the direction
			*visited = uint8(d)
		case 5:
			// special case - multiple visits
			// we've already been here multiple times - noop
		case uint8(d):
			// second visit, in the same orientation
			// whoops, we've already been to this location moving in this direction
			return false
		default:
			// second visit, different orientation
			*visited = 5 // special case, 5 represents multiple visits
		}
		return true
	})

	if !ok {
		return Looping, true
	} else {
		return 0, false
	}
}

type ObstructionCache struct {
	terrain        *grid.Grid[bool]
	neighbourCache *grid.NeighbourCache
	north          []ObstructionCacheValue
	east           []ObstructionCacheValue
	south          []ObstructionCacheValue
	west           []ObstructionCacheValue
}

type ObstructionCacheValue struct {
	init              bool
	prev, obstruction grid.Coord
	isObstructed      bool
}

func MakeObstructionCache(terrain *grid.Grid[bool], neighbourCache *grid.NeighbourCache) ObstructionCache {
	size := terrain.Bounds.Size()
	north := make([]ObstructionCacheValue, size)
	east := make([]ObstructionCacheValue, size)
	south := make([]ObstructionCacheValue, size)
	west := make([]ObstructionCacheValue, size)
	return ObstructionCache{terrain, neighbourCache, north, east, south, west}
}

func (cache *ObstructionCache) val(pos grid.Coord, d grid.Direction) *ObstructionCacheValue {
	switch d {
	case grid.North:
		return &cache.north[int(pos)]
	case grid.East:
		return &cache.east[int(pos)]
	case grid.South:
		return &cache.south[int(pos)]
	case grid.West:
		return &cache.west[int(pos)]
	default:
		panic("Unreachable")
	}
}

func (cache *ObstructionCache) NextObstruction(pos grid.Coord, d grid.Direction) (grid.Coord, grid.Coord, bool) {
	val := cache.val(pos, d)
	if !val.init {
		// is this position an obstruction?
		if cache.terrain.At(pos) {
			// unsupported
			panic("Can't cache an obstruction itself")
		} else {
			// otherwise, check neighbour
			n, inBounds := cache.neighbourCache.Neighbour(pos, d)
			if !inBounds {
				// out of bounds, not obstructed
				val.init = true
				val.prev = pos
				val.isObstructed = false
			} else if cache.terrain.At(n) {
				val.init = true
				val.prev = pos
				val.obstruction = n
				val.isObstructed = true
			} else {
				// recur on neighbour
				val.init = true
				val.prev, val.obstruction, val.isObstructed = cache.NextObstruction(n, d)
			}
		}
	}
	return val.prev, val.obstruction, val.isObstructed
}

func (s *State) NextObstruction(terrain *grid.Grid[bool], obstructionCache *ObstructionCache) (grid.Coord, bool) {
	beforeObstruction, _, isObstructed := obstructionCache.NextObstruction(s.guard.position, s.guard.orientation)
	if terrain.Bounds.IsOnLine(s.guard.position, beforeObstruction, s.extraObstruction) {
		// special case, the extra obstruction is in the way
		beforeExtraObstruction, _ := terrain.Neighbour(s.extraObstruction, s.guard.orientation.Reverse())
		return beforeExtraObstruction, true
	} else {
		return beforeObstruction, isObstructed
	}
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

	result := run(&terrain, &state, &input.obstructionCache)
	util.AssertEqual(OutOfBounds, result)

	return lo.CountBy(state.visited.Values, func(visited uint8) bool { return visited > 0 })
}

func part2(input Input) int {
	terrain, guard := input.terrain, input.guard

	loops := 0

	obstructionCache := &input.obstructionCache

	mainState := initialState(&terrain, guard)
	altState := initialState(&terrain, guard)

	for {
		initialPos := mainState.guard.position
		d := mainState.guard.orientation
		rotated := d.Clockwise()
		lastPassable, _, _ := obstructionCache.NextObstruction(initialPos, d)

		// check all the points between here and the next obstruction OR edge of map, to see if we want to insert a new one
		for pos := initialPos; pos != lastPassable; {
			ahead, inBounds := terrain.Neighbour(pos, d)

			// check if we've already visited this location
			if inBounds && mainState.visited.At(ahead) == 0 {
				// if we put an obstruction ahead, will it result in going off map? if so we can ignore this option
				_, _, isObstructed := obstructionCache.NextObstruction(pos, rotated)
				if isObstructed {
					// try running an alternate simulation with an obstruction here

					// update altState
					altState.guard = mainState.guard
					copy(altState.visited.Values, mainState.visited.Values)
					altState.extraObstruction = ahead

					// run and see if the result is a loop
					res := run(&terrain, &altState, obstructionCache)
					if res == Looping {
						loops++
					}
				}
			}

			pos = ahead
		}

		_, isDone := tick(&terrain, &mainState, obstructionCache)
		if isDone {
			break
		}
	}

	return loops
}
