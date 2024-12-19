package day17

import (
	"adventofcode/util"
	"regexp"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual("5,0,3,5,7,6,1,5,4", part1(input))
	util.AssertEqual(164516454365621, part2(input))
}

type Input struct {
	a, b, c int
	program []uint8
}

func parseInput(input string) Input {
	re := regexp.MustCompile(`\ARegister A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d,]+)\z`)
	match := re.FindStringSubmatch(input)

	a := util.StringToInt(match[1])
	b := util.StringToInt(match[2])
	c := util.StringToInt(match[3])
	program := lo.Map(strings.Split(match[4], ","), func(s string, _ int) uint8 { return uint8(util.StringToInt(s)) })

	return Input{a, b, c, program}
}

func part1(input Input) string {
	comp := MakeSimpleComputer(input)
	comp.Run()
	return comp.OutputString()
}

func part2(input Input) int {
	comp := MakeVariableComputer(input, "a")
	return comp.Solve()
}
