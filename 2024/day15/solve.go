package day15

import (
	"adventofcode/grid"
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
	st     []*Terrain
	active bool
}

func MakeLine(offset int, pos grid.Coord) Line {
	st := []*Terrain{}
	return Line{
		offset: offset,
		tip:    pos,
		st:     st,
		active: true,
	}
}

func (line *Line) Push(pos grid.Coord, canvas *grid.Grid[Terrain]) {
	line.st = append(line.st, canvas.AtMut(pos))
	line.tip = pos
}

func (line *Line) Apply() {
	for j := len(line.st) - 1; j > 0; j-- {
		i := j - 1
		*line.st[j] = *line.st[i]
	}
	*line.st[0] = Empty
}

type ApplyState struct {
	d           grid.Direction
	canvas      *grid.Grid[Terrain]
	lines       []*Line
	activeLines map[int]bool
}

func MakeApplyState(d grid.Direction, canvas *grid.Grid[Terrain]) ApplyState {
	return ApplyState{
		d:           d,
		canvas:      canvas,
		lines:       []*Line{},
		activeLines: make(map[int]bool),
	}
}

func (s *ApplyState) AddLine(offset int, pos grid.Coord) *Line {
	line := MakeLine(offset, pos)
	s.lines = append(s.lines, &line)
	s.activeLines[offset] = true
	return &line
}

func (s *ApplyState) AnyActive() bool {
	return len(s.activeLines) > 0
}

func (s *ApplyState) AddParallelLine(line *Line, inDir grid.Direction) {
	if !s.d.Vertical() {
		return
	}

	// we need to push to the right of this block too
	offset := line.offset
	switch inDir {
	case grid.West:
		offset--
	case grid.East:
		offset++
	}

	if _, ok := s.activeLines[offset]; !ok {
		curr, _ := s.canvas.Neighbour(line.tip, inDir) // parallel step
		ahead, _ := s.canvas.Neighbour(curr, s.d)      // forward step

		line := s.AddLine(offset, curr)
		line.Push(ahead, s.canvas)
	}

}

func applyInstruction(d grid.Direction, robot *grid.Coord, canvas *grid.Grid[Terrain]) {
	state := MakeApplyState(d, canvas)
	state.AddLine(0, *robot)

	for state.AnyActive() {
		for _, line := range state.lines {
			if !line.active {
				continue
			}

			curr := line.tip
			ahead, _ := canvas.Neighbour(curr, d)
			switch canvas.At(ahead) {
			case Empty:
				// we found an empty spot, done
				line.Push(ahead, canvas)
				line.active = false
				delete(state.activeLines, line.offset)
			case Box:
				// keep track of the line of boxes
				line.Push(ahead, canvas)
			case WideBoxLeft:
				// maybe add a parallel push
				state.AddParallelLine(line, grid.East)

				// normal push
				line.Push(ahead, canvas)
			case WideBoxRight:
				// maybe add a parallel push
				state.AddParallelLine(line, grid.West)

				// normal push
				line.Push(ahead, canvas)
			case Wall:
				// no empty space before a wall, nothing we can do
				return
			default:
				panic("Unreachable")
			}
		}
	}

	// push each line of boxes down
	for _, line := range state.lines {
		line.Apply()
	}

	// move robot
	*robot, _ = canvas.Neighbour(*robot, d)
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

func part1(input Input) int {
	robot := input.robot
	canvas := input.canvas.Clone()

	for _, in := range input.instructions {
		applyInstruction(in, &robot, &canvas)
	}

	return score(&canvas)
}

func part2(input Input) int {
	robot, canvas := input.widen()

	for _, in := range input.instructions {
		applyInstruction(in, &robot, &canvas)
	}

	return score(&canvas)
}
