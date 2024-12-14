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
	util.AssertEqual(0, part2(input))
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
		x := calcVal(spec.px, spec.vx, rounds, width)
		y := calcVal(spec.py, spec.vy, rounds, height)

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
	return 0
}
