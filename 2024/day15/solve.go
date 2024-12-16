package day15

import (
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/stack"
	"adventofcode/util"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1437174, part1(input))
	util.AssertEqual(1437468, part2(input))
}

type Input struct {
	robot        grid.Coord
	canvas       grid.Grid[Terrain]
	instructions []grid.Direction
}

type Terrain uint8

const (
	Empty Terrain = iota
	Wall
	Box
	WideBoxLeft
	WideBoxRight
)

func parseInput(input string) Input {
	parts := strings.Split(input, "\n\n")
	util.AssertEqual(2, len(parts))

	var robot grid.Coord
	canvas := grid.Parse[Terrain](parts[0], func(c rune, pos grid.Coord) Terrain {
		switch c {
		case '.':
			return Empty
		case '#':
			return Wall
		case 'O':
			return Box
		case '@':
			robot = pos
			return Empty
		default:
			panic("Unreachable")
		}
	})

	instructions := make([]grid.Direction, 0)
	for _, c := range parts[1] {
		switch c {
		case '^':
			instructions = append(instructions, grid.North)
		case '>':
			instructions = append(instructions, grid.East)
		case 'v':
			instructions = append(instructions, grid.South)
		case '<':
			instructions = append(instructions, grid.West)
		case '\n':
			// noop
		default:
			panic("Unreachable")
		}
	}

	return Input{robot, canvas, instructions}
}

func (input Input) widen() (grid.Coord, grid.Grid[Terrain]) {
	canvas := grid.MakeGrid[Terrain](input.canvas.Bounds.Width*2, input.canvas.Bounds.Height)

	for op, v := range input.canvas.Iter() {
		// pair of positions to set
		p := grid.Coord(int(op) * 2)
		n := grid.Coord(int(p) + 1)

		switch v {
		case Empty:
			// both p and n are empty, do nothing as that's the default value
		case Wall:
			// both p and n are walls
			canvas.Set(p, Wall)
			canvas.Set(n, Wall)
		case Box:
			// double-wide box
			canvas.Set(p, WideBoxLeft)
			canvas.Set(n, WideBoxRight)
		default:
			panic("Unreachable")
		}
	}

	x, y := input.canvas.Bounds.Decompose(input.robot)
	robot, _ := canvas.Bounds.Compose(x*2, y)

	return robot, canvas
}

type Line struct {
	offset int
	tip    grid.Coord
	active bool
}

func MakeLine(offset int, pos grid.Coord) Line {
	return Line{
		offset: offset,
		tip:    pos,
		active: true,
	}
}

func (line *Line) Push(pos grid.Coord, val Terrain, canvas *grid.Grid[Terrain], opQueue *stack.Stack[Operation]) {
	line.tip = pos
	opQueue.Push(Operation{canvas.AtMut(pos), val})
}

type ApplyState struct {
	canvas      *grid.Grid[Terrain]
	robot       grid.Coord
	lines       stack.Stack[Line]
	activeLines set.Set[int]
	opQueue     stack.Stack[Operation]
}

type Operation struct {
	cell *Terrain
	val  Terrain
}

func MakeApplyState(robot grid.Coord, canvas *grid.Grid[Terrain]) ApplyState {
	return ApplyState{
		robot:       robot,
		canvas:      canvas,
		lines:       stack.NewStack[Line](16),
		activeLines: set.NewSet[int](),
		opQueue:     stack.NewStack[Operation](64),
	}
}

func (s *ApplyState) AddLine(offset int, pos grid.Coord) *Line {
	line := MakeLine(offset, pos)
	s.activeLines.Add(offset)
	return s.lines.Push(line)
}

func (s *ApplyState) Push(line *Line, pos grid.Coord, val Terrain) {
	line.Push(pos, val, s.canvas, &s.opQueue)
}

func (s *ApplyState) AnyActive() bool {
	return s.activeLines.Len() > 0
}

func (s *ApplyState) AddParallelLine(line *Line, aheadDir grid.Direction, parallelDir grid.Direction) {
	if !aheadDir.Vertical() {
		return
	}

	// we need to push to the right of this block too
	offset := line.offset
	switch parallelDir {
	case grid.West:
		offset--
	case grid.East:
		offset++
	}

	if !s.activeLines.Contains(offset) {
		curr, _ := s.canvas.Neighbour(line.tip, parallelDir) // parallel step
		ahead, _ := s.canvas.Neighbour(curr, aheadDir)       // forward step

		line := s.AddLine(offset, curr)

		// first push slot will be emptied
		line.Push(ahead, Empty, s.canvas, &s.opQueue)
	}
}

func (s *ApplyState) Clear() {
	s.lines.Clear()
	s.activeLines.Clear()
	s.opQueue.Clear()
}

func (state *ApplyState) ApplyInstruction(d grid.Direction) {
	state.AddLine(0, state.robot)

	for state.AnyActive() {
		for line := range state.lines.Iter() {
			if !line.active {
				continue
			}

			curr := line.tip
			currVal := state.canvas.At(curr)

			ahead, _ := state.canvas.Neighbour(curr, d)
			aheadVal := state.canvas.At(ahead)

			switch aheadVal {
			case Empty:
				// we found an empty spot, done
				state.Push(line, ahead, currVal)
				line.active = false
				state.activeLines.Remove(line.offset)
			case Box:
				// keep track of the line of boxes
				state.Push(line, ahead, currVal)
			case WideBoxLeft:
				// maybe add a parallel push
				state.AddParallelLine(line, d, grid.East)

				// normal push
				state.Push(line, ahead, currVal)
			case WideBoxRight:
				// maybe add a parallel push
				state.AddParallelLine(line, d, grid.West)

				// normal push
				state.Push(line, ahead, currVal)
			case Wall:
				// no empty space before a wall, nothing we can do
				return
			default:
				panic("Unreachable")
			}
		}
	}

	// push each line of boxes down
	for op := range state.opQueue.Iter() {
		*op.cell = op.val
	}

	// move robot
	state.robot, _ = state.canvas.Neighbour(state.robot, d)
}

func score(canvas *grid.Grid[Terrain]) int {
	score := 0
	for pos, v := range canvas.Iter() {
		if v == Box || v == WideBoxLeft {
			x, y := canvas.Bounds.Decompose(pos)
			score += y*100 + x
		}
	}
	return score
}

//lint:ignore U1000 for debugging
func print(robot *grid.Coord, canvas *grid.Grid[Terrain]) {
	canvas.Print(func(v Terrain, pos grid.Coord) rune {
		if pos == *robot {
			return '@'
		} else {
			switch v {
			case Empty:
				return '.'
			case Wall:
				return '#'
			case Box:
				return 'O'
			case WideBoxLeft:
				return '['
			case WideBoxRight:
				return ']'
			default:
				panic("Unreachable")
			}
		}
	})
}

func solve(instructions []grid.Direction, robot grid.Coord, canvas grid.Grid[Terrain]) int {
	state := MakeApplyState(robot, &canvas)

	for _, in := range instructions {
		state.ApplyInstruction(in)
		state.Clear()
	}

	return score(&canvas)
}

func part1(input Input) int {
	robot := input.robot
	canvas := input.canvas.Clone()
	return solve(input.instructions, robot, canvas)
}

func part2(input Input) int {
	robot, canvas := input.widen()
	return solve(input.instructions, robot, canvas)
}
