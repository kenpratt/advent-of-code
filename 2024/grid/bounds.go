package grid

import (
	"adventofcode/util"
	"fmt"
)

type Bounds struct {
	Width  int
	Height int
}

func MakeBounds(width, height int) Bounds {
	return Bounds{width, height}
}

func (bounds *Bounds) Size() int {
	return bounds.Width * bounds.Height
}

func (bounds *Bounds) Within(x, y int) bool {
	return y >= 0 && y < bounds.Height && x >= 0 && x < bounds.Width
}

func (bounds *Bounds) AddOffset(c Coord, o Offset) (Coord, bool) {
	x1, y1 := bounds.Decompose(c)
	x2, y2 := o.X, o.Y
	x := x1 + x2
	y := y1 + y2
	return bounds.Compose(x, y)
}

func (bounds *Bounds) Subtract(c1, c2 Coord) Offset {
	x1, y1 := bounds.Decompose(c1)
	x2, y2 := bounds.Decompose(c2)
	x := x1 - x2
	y := y1 - y2
	return MakeOffset(x, y)
}

func (bounds *Bounds) SubtractOffset(c Coord, o Offset) (Coord, bool) {
	x1, y1 := bounds.Decompose(c)
	x2, y2 := o.X, o.Y
	x := x1 - x2
	y := y1 - y2
	return bounds.Compose(x, y)
}

func (bounds *Bounds) Compose(x, y int) (Coord, bool) {
	if bounds.Within(x, y) {
		return Coord(y*bounds.Width + x), true
	} else {
		return Coord(-1), false
	}
}

func (bounds *Bounds) Decompose(pos Coord) (int, int) {
	x := int(pos) % bounds.Width
	y := int(pos) / bounds.Width
	return x, y
}

func (bounds *Bounds) Neighbour(pos Coord, d Direction) (Coord, bool) {
	return bounds.MoveInDirection(pos, d, 1)
}

func (bounds *Bounds) MoveInDirection(pos Coord, d Direction, distance int) (Coord, bool) {
	x, y := bounds.Decompose(pos)

	switch d {
	case North:
		y -= distance
	case East:
		x += distance
	case South:
		y += distance
	case West:
		x -= distance
	default:
		panic(fmt.Sprintf("Unknown direction: %v", d))
	}

	return bounds.Compose(x, y)
}

func (bounds *Bounds) ManhattanDistance(p1 Coord, p2 Coord) int {
	x1, y1 := bounds.Decompose(p1)
	x2, y2 := bounds.Decompose(p2)
	return util.AbsDiff(x1, x2) + util.AbsDiff(y1, y2)
}

func (bounds *Bounds) IsOnLine(p1 Coord, p2 Coord, m Coord) bool {
	x1, y1 := bounds.Decompose(p1)
	x2, y2 := bounds.Decompose(p2)
	xm, ym := bounds.Decompose(m)

	if x1 == xm && xm == x2 {
		// same x, check if ym is in between
		if y1 <= y2 {
			return y1 <= ym && ym <= y2
		} else {
			return y2 <= ym && ym <= y1
		}
	} else if y1 == ym && ym == y2 {
		// same y, check if xm is in between
		if x1 <= x2 {
			return x1 <= xm && xm <= x2
		} else {
			return x2 <= xm && xm <= x1
		}
	} else {
		return false
	}
}
