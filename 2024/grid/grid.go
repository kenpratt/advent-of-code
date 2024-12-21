package grid

import (
	"adventofcode/util"
	"fmt"
	"iter"
	"strings"
)

// Coord is an int, except all the functions assume or ensure it is within the bounds
type Coord int

type Grid[T any] struct {
	Bounds Bounds
	Values []T
}

func Parse[T any](input string, parse func(rune, Coord) T) Grid[T] {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := Bounds{Width: width, Height: height}
	values := make([]T, bounds.Size())

	for y, line := range lines {
		for x, char := range line {
			pos, ok := bounds.Compose(x, y)
			util.AssertEqual(true, ok)
			val := parse(char, pos)
			values[int(pos)] = val
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
			pos, ok := bounds.Compose(x, y)
			util.AssertEqual(true, ok)
			parse(char, pos)
		}
	}

	return bounds
}

func MakeGrid[T any](width, height int) Grid[T] {
	bounds := Bounds{Width: width, Height: height}
	values := make([]T, bounds.Size())
	return Grid[T]{bounds, values}
}

func (grid *Grid[T]) Clone() Grid[T] {
	values := make([]T, len(grid.Values))
	copy(values, grid.Values)
	return Grid[T]{
		Bounds: grid.Bounds,
		Values: values,
	}
}

func (grid *Grid[T]) Clear() {
	clear(grid.Values)
}

func (grid *Grid[T]) Len() int {
	return len(grid.Values)
}

func (grid *Grid[T]) At(pos Coord) T {
	return grid.Values[pos]
}

func (grid *Grid[T]) AtMut(pos Coord) *T {
	return &grid.Values[pos]
}

func (grid Grid[T]) Set(pos Coord, value T) {
	grid.Values[int(pos)] = value
}

func (grid *Grid[T]) Iter() iter.Seq2[Coord, T] {
	return func(yield func(Coord, T) bool) {
		for i, v := range grid.Values {
			if !yield(Coord(i), v) {
				return
			}
		}
	}
}

func (grid *Grid[T]) Compose(x, y int) (Coord, bool) {
	return grid.Bounds.Compose(x, y)
}

func (grid *Grid[T]) Neighbour(pos Coord, direction Direction) (Coord, bool) {
	return grid.Bounds.MoveInDirection(pos, direction, 1)
}

func (grid *Grid[T]) IterNeighbours(pos Coord) iter.Seq[Coord] {
	return func(yield func(Coord) bool) {
		for _, d := range Directions() {
			n, ok := grid.Neighbour(pos, d)
			if ok {
				if !yield(n) {
					return
				}
			}
		}
	}
}

func (grid *Grid[T]) AddOffset(c Coord, o Offset) (Coord, bool) {
	return grid.Bounds.AddOffset(c, o)
}

func (grid *Grid[T]) Subtract(c1, c2 Coord) Offset {
	return grid.Bounds.Subtract(c1, c2)
}

func (grid *Grid[T]) SubtractOffset(c Coord, o Offset) (Coord, bool) {
	return grid.Bounds.SubtractOffset(c, o)
}

func (grid *Grid[T]) Print(f func(T, Coord) rune) {
	for y := 0; y < grid.Bounds.Height; y++ {
		for x := 0; x < grid.Bounds.Width; x++ {
			pos, _ := grid.Bounds.Compose(x, y)
			c := f(grid.At(pos), pos)
			fmt.Printf("%c", c)
		}
		fmt.Println()
	}
}

func (grid *Grid[T]) ForLinearPath(from Coord, to Coord, do func(Coord) bool) bool {
	x1, y1 := grid.Bounds.Decompose(from)
	x2, y2 := grid.Bounds.Decompose(to)

	var steps, incr int
	if y1 == y2 {
		// horizontal line
		if x1 < x2 {
			steps = x2 - x1
			incr = 1
		} else {
			steps = x1 - x2
			incr = -1
		}
	} else if x1 == x2 {
		// vertical line
		if y1 < y2 {
			steps = y2 - y1
			incr = grid.Bounds.Width
		} else {
			steps = y1 - y2
			incr = -grid.Bounds.Width
		}
	} else {
		panic(fmt.Sprintf("IterLine expects the two points to be on a straight line: %v, %v", from, to))
	}

	for i := 0; i < steps; i++ {
		ok := do(Coord(int(from) + (i+1)*incr))
		if !ok {
			return false
		}
	}

	return true
}
