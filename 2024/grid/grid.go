package grid

import "fmt"

type Coord struct {
	X int
	Y int
}

func MakeCoord(x int, y int) Coord {
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

func Clone[T any](grid *Grid[T]) Grid[T] {
	values := make([]T, len(grid.Values))
	copy(values, grid.Values)
	return Grid[T]{
		Bounds: grid.Bounds,
		Values: values,
	}
}

func CoordToIndex(bounds *Bounds, pos *Coord) int {
	return pos.Y*bounds.Width + pos.X
}

func IndexToCoord(bounds *Bounds, i int) Coord {
	y := i / bounds.Width
	x := i % bounds.Width
	return MakeCoord(x, y)
}

func InBounds(bounds *Bounds, pos *Coord) bool {
	return pos.Y >= 0 && pos.Y < bounds.Height && pos.X >= 0 && pos.X < bounds.Width
}

func AddCoords(c1 *Coord, c2 *Coord) Coord {
	x := c1.X + c2.X
	y := c1.Y + c2.Y
	return Coord{x, y}
}

func SubtractCoords(c1 *Coord, c2 *Coord) Coord {
	x := c1.X - c2.X
	y := c1.Y - c2.Y
	return Coord{x, y}
}

func At[T any](grid *Grid[T], pos *Coord) (*T, bool) {
	if InBounds(&grid.Bounds, pos) {
		index := CoordToIndex(&grid.Bounds, pos)
		return &grid.Values[index], true
	} else {
		var noop T
		return &noop, false
	}
}

func Set[T any](grid *Grid[T], pos *Coord, value T) bool {
	if InBounds(&grid.Bounds, pos) {
		index := CoordToIndex(&grid.Bounds, pos)
		grid.Values[index] = value
		return true
	} else {
		return false
	}
}

func DiagonalOffsets() [8]Coord {
	return [...]Coord{{X: -1, Y: -1}, {X: 0, Y: -1}, {X: 1, Y: -1}, {X: -1, Y: 0}, {X: 1, Y: 0}, {X: -1, Y: 1}, {X: 0, Y: 1}, {X: 1, Y: 1}}
}

type Direction int

const (
	North Direction = iota + 1
	East
	South
	West
)

func TurnRight(d Direction) Direction {
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

func TurnLeft(d Direction) Direction {
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

func MoveInDirection(pos Coord, d Direction, distance int) Coord {
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
