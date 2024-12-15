package day15

import (
	"adventofcode/grid"
	"adventofcode/stack"
	"adventofcode/util"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1437174, part1(input))
	util.AssertEqual(0, part2(input))
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

func applyInstruction(d grid.Direction, robot *grid.Coord, canvas *grid.Grid[Terrain]) {
	boxes := stack.NewStack[grid.Coord](8)
	last := *robot
	done := false
	for !done {
		curr, _ := canvas.Neighbour(last, d)
		switch canvas.At(curr) {
		case Empty:
			// we found an empty spot, done
			last = curr
			done = true
		case Box:
			// keep track of the line of boxes
			boxes.Push(curr)
			last = curr
		case Wall:
			// no empty space before a wall, nothing we can do
			return
		}
	}

	// push the line of boxes down
	for boxes.Len() > 0 {
		canvas.Set(last, Box)
		last = boxes.Pop()
	}

	// move robot
	canvas.Set(last, Empty)
	*robot = last
}

func score(canvas *grid.Grid[Terrain]) int {
	score := 0
	for pos, v := range canvas.Iter() {
		if v == Box {
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
			default:
				panic("Unreachable")
			}
		}
	})
}

func part1(input Input) int {
	robot := input.robot
	canvas := input.canvas

	for _, in := range input.instructions {
		applyInstruction(in, &robot, &canvas)
		// print(&robot, &canvas)
	}

	return score(&canvas)
}

func part2(input Input) int {
	return 0
}
