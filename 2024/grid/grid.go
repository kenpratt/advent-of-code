package grid

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

func CoordToIndex(bounds Bounds, pos Coord) int {
	return pos.Y*bounds.Width + pos.X
}

func InBounds(bounds Bounds, pos Coord) bool {
	return pos.Y >= 0 && pos.Y < bounds.Height && pos.X >= 0 && pos.X < bounds.Width
}

func AddCoords(c1 Coord, c2 Coord) Coord {
	x := c1.X + c2.X
	y := c1.Y + c2.Y
	return Coord{x, y}
}

func At[T any](grid Grid[T], pos Coord) (T, bool) {
	if InBounds(grid.Bounds, pos) {
		index := CoordToIndex(grid.Bounds, pos)
		return grid.Values[index], true
	} else {
		var noop T
		return noop, false
	}
}

func DiagonalOffsets() [8]Coord {
	return [...]Coord{{X: -1, Y: -1}, {X: 0, Y: -1}, {X: 1, Y: -1}, {X: -1, Y: 0}, {X: 1, Y: 0}, {X: -1, Y: 1}, {X: 0, Y: 1}, {X: 1, Y: 1}}
}
