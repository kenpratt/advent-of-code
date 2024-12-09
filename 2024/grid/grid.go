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

func (grid *Grid[T]) Clone() Grid[T] {
	values := make([]T, len(grid.Values))
	copy(values, grid.Values)
	return Grid[T]{
		Bounds: grid.Bounds,
		Values: values,
	}
}

func (bounds *Bounds) CoordToIndex(pos *Coord) int {
	return pos.Y*bounds.Width + pos.X
}

func (bounds *Bounds) IndexToCoord(i int) Coord {
	y := i / bounds.Width
	x := i % bounds.Width
	return MakeCoord(x, y)
}

func (bounds *Bounds) Within(pos *Coord) bool {
	return pos.Y >= 0 && pos.Y < bounds.Height && pos.X >= 0 && pos.X < bounds.Width
}

func (c1 *Coord) Add(c2 *Coord) Coord {
	x := c1.X + c2.X
	y := c1.Y + c2.Y
	return Coord{x, y}
}

func (c1 *Coord) Subtract(c2 *Coord) Coord {
	x := c1.X - c2.X
	y := c1.Y - c2.Y
	return Coord{x, y}
}

func (grid *Grid[T]) At(pos *Coord) (*T, bool) {
	if grid.Bounds.Within(pos) {
		index := grid.Bounds.CoordToIndex(pos)
		return &grid.Values[index], true
	} else {
		var noop T
		return &noop, false
	}
}

func (grid *Grid[T]) Set(pos *Coord, value T) bool {
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

type Direction int

const (
	North Direction = iota + 1
	East
	South
	West
)

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
