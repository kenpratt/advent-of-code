package grid

import "fmt"

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

func (d Direction) Horizontal() bool {
	switch d {
	case West, East:
		return true
	default:
		return false
	}
}

func (d Direction) Vertical() bool {
	switch d {
	case North, South:
		return true
	default:
		return false
	}
}

func (d Direction) ToString() string {
	switch d {
	case North:
		return "North"
	case West:
		return "West"
	case South:
		return "South"
	case East:
		return "East"
	default:
		panic(fmt.Sprintf("Unknown direction: %v", d))
	}
}
