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
	util.AssertEqual(1667, part2(input))
}

func parseInput(input string) []uint32 {
	lines := strings.Split(input, "\n")
	return lo.Map(lines, func(s string, _ int) uint32 { return uint32(util.StringToInt(s)) })
}

func nthSecretNumber(initial uint32, nth int) uint32 {
	last := initial
	for i := 0; i < nth; i++ {
		curr := currSecretNumber(last)
		last = curr
	}
	return last
}

// 64       = 2^6
// 32       = 2^5
// 2048     = 2^11
// 16777216 = 2^24
func currSecretNumber(val uint32) uint32 {
	// modulo 2^24 is the same as bitwise AND 2^24-1 (23 1's)

	// mult by 64 (lshift 6), bitwise XOR with self, modulo
	val = (val ^ (val << 6)) & 16777215

	// div by 32 (rshift 5), bitwise XOR with self, modulo
	val = (val ^ (val >> 5)) & 16777215

	// mult by 2048 (lshift 11), bitwise XOR with self, modulo
	val = (val ^ (val << 11)) & 16777215

	return val
}

func calculatePriceChangeSequences(initial uint32, rounds int) map[[4]int8]uint8 {
	sequences := make(map[[4]int8]uint8)

	lastNumber := initial
	lastPrice := uint8(initial % 10)
	var seq [4]int8

	for i := 1; i <= rounds; i++ {
		currNumber := currSecretNumber(lastNumber)
		currPrice := uint8(currNumber % 10)
		change := int8(currPrice) - int8(lastPrice)
		seq[3] = change

		if i > 3 {
			// add to sequences
			if _, ok := sequences[seq]; !ok {
				// only record the first time each sequence is seen
				sequences[seq] = currPrice
			}
		}

		lastNumber = currNumber
		lastPrice = currPrice
		copy(seq[0:3], seq[1:4])
	}

	return sequences
}

func part1(initial []uint32) int {
	return lo.SumBy(initial, func(n uint32) int { return int(nthSecretNumber(n, 2000)) })
}

func part2(initial []uint32) int {
	combined := make(map[[4]int8]uint16)

	for _, n := range initial {
		sequences := calculatePriceChangeSequences(n, 2000)
		for seq, bananas := range sequences {
			combined[seq] += uint16(bananas)
		}
	}

	res := uint16(0)
	for _, bananas := range combined {
		res = max(res, bananas)
	}
	return int(res)
}
