package day14

import (
	"adventofcode/util"
	"regexp"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(214400550, part1(input))
	util.AssertEqual(8149, part2(input))
}

type RobotSpec struct {
	px, py, vx, vy int
}

func parseInput(input string) []RobotSpec {
	re := regexp.MustCompile(`p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)`)
	matches := re.FindAllStringSubmatch(input, -1)

	return lo.Map(matches, func(m []string, _ int) RobotSpec {
		return RobotSpec{
			px: util.StringToInt(m[1]),
			py: util.StringToInt(m[2]),
			vx: util.StringToInt(m[3]),
			vy: util.StringToInt(m[4]),
		}
	})
}

func calcVal(p, v, t, len int) int {
	r := (p + v*t) % len
	if r < 0 {
		r += (r/len + 1) * len
	}
	return r
}

func calcPos(spec RobotSpec, t int, width int, height int) (int, int) {
	x := calcVal(spec.px, spec.vx, t, width)
	y := calcVal(spec.py, spec.vy, t, height)
	return x, y
}

func part1(specs []RobotSpec, extra ...int) int {
	rounds := 100
	width := 101
	height := 103

	// for tests
	if len(extra) > 0 {
		width = extra[0]
		height = extra[1]
	}

	ul, ur, bl, br := 0, 0, 0, 0

	mx := width / 2
	my := height / 2

	for _, spec := range specs {
		x, y := calcPos(spec, rounds, width, height)

		switch {
		case x < mx && y < my:
			ul++
		case x > mx && y < my:
			ur++
		case x < mx && y > my:
			bl++
		case x > mx && y > my:
			br++
		}
	}

	return ul * ur * bl * br
}

func part2(specs []RobotSpec) int {
	width := 101
	height := 103

	state := State{
		round:  0,
		width:  width,
		height: height,
		robots: specs,
		xCount: make([]int, width),
		yCount: make([]int, height),
	}

	// search until we find a score that is way bigger than the start
	target := state.tick() * 5 / 2
	for {
		score := state.tick()
		if score >= target {
			// winner winner chicken dinner
			return state.round
		}
	}
}

type State struct {
	round  int
	width  int
	height int
	robots []RobotSpec
	xCount []int
	yCount []int
}

func (s *State) tick() int {
	// clear previous state
	clear(s.xCount)
	clear(s.yCount)

	// update robots
	for i := range s.robots {
		r := &s.robots[i]

		// update position
		r.px = (r.px + r.vx + s.width) % s.width
		r.py = (r.py + r.vy + s.height) % s.height

		// record counts per row/col
		s.xCount[r.px]++
		s.yCount[r.py]++
	}

	s.round++

	// calculate score as highest density column + row
	return lo.Max(s.xCount) + lo.Max(s.yCount)
}
