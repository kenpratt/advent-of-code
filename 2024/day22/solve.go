package day22

import (
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(14273043166, part1(input))
	util.AssertEqual(0, part2(input))
}

func parseInput(input string) []uint32 {
	lines := strings.Split(input, "\n")
	return lo.Map(lines, func(s string, _ int) uint32 { return uint32(util.StringToInt(s)) })
}

func nthSecretNumber(initial uint32, nth int) uint32 {
	curr := initial
	for i := 0; i < nth; i++ {
		next := nextSecretNumber(curr)
		curr = next
	}
	return curr
}

// 64       = 2^6
// 32       = 2^5
// 2048     = 2^11
// 16777216 = 2^24
func nextSecretNumber(val uint32) uint32 {
	// modulo 2^24 is the same as bitwise AND 2^24-1 (23 1's)

	// mult by 64 (lshift 6), bitwise XOR with self, modulo
	val = (val ^ (val << 6)) & 16777215

	// div by 32 (rshift 5), bitwise XOR with self, modulo
	val = (val ^ (val >> 5)) & 16777215

	// mult by 2048 (lshift 11), bitwise XOR with self, modulo
	val = (val ^ (val << 11)) & 16777215

	return val
}

func part1(initial []uint32) int {
	return lo.SumBy(initial, func(n uint32) int { return int(nthSecretNumber(n, 2000)) })
}

func part2(starting []uint32) int {
	return 0
}
