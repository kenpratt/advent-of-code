package day11

import (
	"adventofcode/util"
	"math"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(217812, part1(input))
	util.AssertEqual(259112729857522, part2(input))
}

func parseInput(input string) []int {
	parts := strings.Fields(input)
	return lo.Map(parts, func(s string, _ int) int { return util.StringToInt(s) })
}

func finalStoneCount(stones []int, rounds int) int {
	stoneCounts := make(map[int]int)
	for _, s := range stones {
		stoneCounts[s]++
	}

	for i := 0; i < rounds; i++ {
		stoneCounts = tick(stoneCounts)
	}

	sum := 0
	for _, n := range stoneCounts {
		sum += n
	}
	return sum
}

func tick(stoneCounts map[int]int) map[int]int {
	// estimate size of result to avoid extra allocs
	sizeEstimate := len(stoneCounts) + int(math.Sqrt(float64(len(stoneCounts))))
	result := make(map[int]int, sizeEstimate)

	for stone, count := range stoneCounts {
		digits := util.NumDigits(stone)
		switch {
		case digits == 0:
			result[1] += count
		case digits%2 == 0:
			left, right := util.SplitInts(stone, digits/2)
			result[left] += count
			result[right] += count
		default:
			result[stone*2024] += count
		}
	}

	return result
}

func part1(stones []int) int {
	return finalStoneCount(stones, 25)
}

func part2(stones []int) int {
	return finalStoneCount(stones, 75)
}
