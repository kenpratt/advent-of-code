package grid

type Offset struct {
	X int
	Y int
}

func MakeOffset(x, y int) Offset {
	return Offset{
		X: x,
		Y: y,
	}
}

func DiagonalOffsets() [8]Offset {
	return [...]Offset{
		{X: -1, Y: -1},
		{X: 0, Y: -1},
		{X: 1, Y: -1},
		{X: -1, Y: 0},
		{X: 1, Y: 0},
		{X: -1, Y: 1},
		{X: 0, Y: 1},
		{X: 1, Y: 1},
	}
}
