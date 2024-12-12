package grid

import (
	"fmt"
	"strings"
)

type Coord struct {
	X int
	Y int
}

func MakeCoord(x, y int) Coord {
	return Coord{X: x, Y: y}
}

type Bounds struct {
	Width  int
	Height int
}

type Grid[T any] struct {
	Bounds Bounds
	Values []T
}

func Parse[T any](input string, parse func(rune, Coord) T) Grid[T] {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := Bounds{Width: width, Height: height}
	values := make([]T, width*height)

	for y, line := range lines {
		for x, char := range line {
			pos := MakeCoord(x, y)
			i := bounds.CoordToIndex(pos)
			val := parse(char, pos)
			values[i] = val
		}
	}

	return Grid[T]{bounds, values}
}

// just make Bounds and call the func per pos, don't actually build a grid
func ParseBoundsAndCoords(input string, parse func(rune, Coord)) Bounds {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := Bounds{Width: width, Height: height}

	for y, line := range lines {
		for x, char := range line {
			pos := MakeCoord(x, y)
			parse(char, pos)
		}
	}

	return bounds
}

func (grid *Grid[T]) Clone() Grid[T] {
	values := make([]T, len(grid.Values))
	copy(values, grid.Values)
	return Grid[T]{
		Bounds: grid.Bounds,
		Values: values,
	}
}

func (grid *Grid[T]) Len() int {
	return len(grid.Values)
}

func (grid *Grid[T]) NeighbourForIndex(i int, direction Direction) (int, bool) {
	// TODO try to implement with a simpler math approach
	c := grid.Bounds.IndexToCoord(i)
	n := c.MoveInDirection(direction, 1)
	if grid.Bounds.Within(n) {
		return grid.Bounds.CoordToIndex(n), true
	} else {
		return -1, false
	}
}

func (bounds Bounds) CoordToIndex(pos Coord) int {
	return pos.Y*bounds.Width + pos.X
}

func (bounds Bounds) IndexToCoord(i int) Coord {
	y := i / bounds.Width
	x := i % bounds.Width
	return MakeCoord(x, y)
}

func (bounds Bounds) Within(pos Coord) bool {
	return pos.Y >= 0 && pos.Y < bounds.Height && pos.X >= 0 && pos.X < bounds.Width
}

func (c1 Coord) Add(c2 Coord) Coord {
	x := c1.X + c2.X
	y := c1.Y + c2.Y
	return Coord{x, y}
}

func (c1 Coord) Subtract(c2 Coord) Coord {
	x := c1.X - c2.X
	y := c1.Y - c2.Y
	return Coord{x, y}
}

func (grid *Grid[T]) At(pos Coord) (T, bool) {
	if grid.Bounds.Within(pos) {
		index := grid.Bounds.CoordToIndex(pos)
		return grid.Values[index], true
	} else {
		var noop T
		return noop, false
	}
}

func (grid *Grid[T]) AtMut(pos Coord) (*T, bool) {
	if grid.Bounds.Within(pos) {
		index := grid.Bounds.CoordToIndex(pos)
		return &grid.Values[index], true
	} else {
		var noop T
		return &noop, false
	}
}

func (grid Grid[T]) Set(pos Coord, value T) bool {
	if grid.Bounds.Within(pos) {
		index := grid.Bounds.CoordToIndex(pos)
		grid.Values[index] = value
		return true
	} else {
		return false
	}
}

func DiagonalOffsets() [8]Coord {
	return [...]Coord{{X: -1, Y: -1}, {X: 0, Y: -1}, {X: 1, Y: -1}, {X: -1, Y: 0}, {X: 1, Y: 0}, {X: -1, Y: 1}, {X: 0, Y: 1}, {X: 1, Y: 1}}
}

type Direction uint8

const (
	North Direction = iota + 1
	East
	South
	West
)

func Directions() [4]Direction {
	return [4]Direction{North, East, South, West}
}

func (d Direction) Clockwise() Direction {
	switch d {
	case North:
		return East
	case East:
		return South
	case South:
		return West
	case West:
		return North
	default:
		panic(fmt.Sprintf("Unknown direction: %v", d))
	}
}

func (d Direction) CounterClockwise() Direction {
	switch d {
	case North:
		return West
	case West:
		return South
	case South:
		return East
	case East:
		return North
	default:
		panic(fmt.Sprintf("Unknown direction: %v", d))
	}
}

func (pos Coord) MoveInDirection(d Direction, distance int) Coord {
	switch d {
	case North:
		return Coord{X: pos.X, Y: pos.Y - distance}
	case East:
		return Coord{X: pos.X + distance, Y: pos.Y}
	case South:
		return Coord{X: pos.X, Y: pos.Y + distance}
	case West:
		return Coord{X: pos.X - distance, Y: pos.Y}
	default:
		panic(fmt.Sprintf("Unknown direction: %v", d))
	}
}
