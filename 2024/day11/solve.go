package day11

import (
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(217812, part1(input))
	util.AssertEqual(0, part2(input))
}

func parseInput(input string) []int {
	parts := strings.Fields(input)
	return lo.Map(parts, func(s string, _ int) int { return util.StringToInt(s) })
}

func tick(stones []int) []int {
	result := make([]int, 0)
	for _, stone := range stones {
		digits := numDigits(stone)
		switch {
		case digits == 0:
			result = append(result, 1)
		case digits%2 == 0:
			left, right := split(stone, digits/2)
			result = append(result, left)
			result = append(result, right)
		default:
			result = append(result, stone*2024)
		}
	}
	return result
}

func numDigits(val int) int {
	digits := 0
	for val > 0 {
		val /= 10
		digits++
	}
	return digits
}

func split(val, digits int) (int, int) {
	left := val
	right := 0

	place := 1
	for digits > 0 {
		v := left % 10
		left /= 10
		right += v * place

		digits--
		place *= 10
	}

	return left, right
}

func part1(stones []int) int {
	for i := 0; i < 25; i++ {
		stones = tick(stones)
	}

	return len(stones)
}

func part2(stones []int) int {
	return 0
}
