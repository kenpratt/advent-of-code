package day21

import (
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(219366, part1(input))
	util.AssertEqual(0, part2(input))
}

func parseInput(input string) [][4]Numpad {
	lines := strings.Split(input, "\n")

	return lo.Map(lines, func(s string, _ int) [4]Numpad {
		code := [4]Numpad{}
		for i, c := range s {
			if c == 'A' {
				code[i] = NumpadA
			} else {
				code[i] = Numpad(util.RuneToInt(c))
			}
		}
		return code
	})
}

type Pad interface {
	row() uint8
	col() uint8
	gapRow() uint8
}

type Numpad uint8

const (
	Zero Numpad = iota
	One
	Two
	Three
	Four
	Five
	Six
	Seven
	Eight
	Nine
	NumpadA
)

func (n Numpad) row() uint8 {
	switch n {
	case Zero, NumpadA:
		return 0
	case One, Two, Three:
		return 1
	case Four, Five, Six:
		return 2
	case Seven, Eight, Nine:
		return 3
	default:
		panic("Unreachable")
	}
}

func (n Numpad) col() uint8 {
	switch n {
	case One, Four, Seven:
		return 0
	case Zero, Two, Five, Eight:
		return 1
	case NumpadA, Three, Six, Nine:
		return 2
	default:
		panic("Unreachable")
	}
}

func (n Numpad) gapRow() uint8 {
	return 0
}

type Dpad uint8

const (
	Up Dpad = iota + 1
	Down
	Left
	Right
	DpadA
)

type DpadStep struct {
	button Dpad
	times  uint8
}

func (n Dpad) gapRow() uint8 {
	return 1
}

func (n Dpad) row() uint8 {
	switch n {
	case Left, Down, Right:
		return 0
	case Up, DpadA:
		return 1
	default:
		panic("Unreachable")
	}
}

func (n Dpad) col() uint8 {
	switch n {
	case Left:
		return 0
	case Up, Down:
		return 1
	case Right, DpadA:
		return 2
	default:
		panic("Unreachable")
	}
}

func robotPath(from, to Pad) []DpadStep {
	fromRow := from.row()
	toRow := to.row()
	fromCol := from.col()
	toCol := to.col()

	steps := make([]DpadStep, 0)

	var vertical DpadStep
	if toRow >= fromRow {
		vertical = DpadStep{Up, toRow - fromRow}
	} else {
		vertical = DpadStep{Down, fromRow - toRow}
	}

	var horizontal DpadStep
	if toCol >= fromCol {
		horizontal = DpadStep{Right, toCol - fromCol}
	} else {
		horizontal = DpadStep{Left, fromCol - toCol}
	}

	// figure out which direction to move first
	var verticalFirst bool
	if horizontal.button == Left && horizontal.times > 0 {
		// if we need to move left, prefer moving left first as it's expensive, unless it would put us in a hole
		if toCol == 0 && fromRow == from.gapRow() {
			// moving left first would put us in a hole
			verticalFirst = true
		} else {
			verticalFirst = false
		}
	} else if vertical.button == Down && vertical.times > 0 {
		// similarly for moving down, prefer it first unless it puts us in the hole
		if toRow == from.gapRow() && fromCol == 0 {
			// moving down first would put us in a hole
			verticalFirst = false
		} else {
			verticalFirst = true
		}
	} else {
		// doesn't matter
		verticalFirst = true
	}

	if verticalFirst {
		if vertical.times > 0 {
			steps = append(steps, vertical)
		}
		if horizontal.times > 0 {
			steps = append(steps, horizontal)
		}
	} else {
		if horizontal.times > 0 {
			steps = append(steps, horizontal)
		}
		if vertical.times > 0 {
			steps = append(steps, vertical)
		}
	}

	// then press A once
	steps = append(steps, DpadStep{DpadA, 1})

	return steps
}

func shortestNumpadSequence(code [4]Numpad) int {
	res := 0

	// always start from A
	curr := NumpadA
	for _, next := range code {
		firstRobot := robotPath(curr, next)
		res += shortestDpadSequence(firstRobot, 2)
		curr = next
	}

	return res
}

// shortest sequence for a two robot setup
func shortestDpadSequence(steps []DpadStep, nesting uint8) int {
	res := 0

	// always start from A
	curr := DpadA
	for _, step := range steps {
		next := step.button

		var press int
		// figure out if we still have more robots
		if nesting > 0 {
			// robot cost is a dpad sequence
			secondRobot := robotPath(curr, next)
			press = shortestDpadSequence(secondRobot, nesting-1)
		} else {
			// human cost is 1, can just press the button
			press = 1
		}
		res += press

		// and the button presses
		if step.times > 1 {
			res += int(step.times - 1)
		}

		curr = next
	}

	return res
}

func numericCode(code [4]Numpad) int {
	res := 0
	for _, n := range code[:3] {
		util.AssertEqual(false, n == NumpadA)
		res *= 10
		res += int(n)
	}
	return res
}

func part1(codes [][4]Numpad) int {
	res := 0
	for _, c := range codes {
		seq := shortestNumpadSequence(c)
		code := numericCode(c)
		cost := seq * code
		res += cost
	}
	return res
}

func part2(codes [][4]Numpad) int {
	return 0
}
