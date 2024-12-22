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

	// div by 32 (rshift 5), bitwise XOR with self, don't need to modulo
	val = (val ^ (val >> 5))

	// mult by 2048 (lshift 11), bitwise XOR with self, modulo
	val = (val ^ (val << 11)) & 16777215

	return val
}

const sequencesSize = 160000 // 20^4

func sequenceIndex(seq [4]int8) uint32 {
	// convert seq to a base 20 number
	d0 := uint32(seq[0] + 10)
	d1 := uint32(seq[1] + 10)
	d2 := uint32(seq[2] + 10)
	d3 := uint32(seq[3] + 10)
	return d0*8000 + d1*400 + d2*20 + d3
}

func calculatePriceChangeSequences(initial uint32, rounds int, combined *[sequencesSize]uint16, seen *[sequencesSize]bool) {
	lastNumber := initial
	lastPrice := uint8(initial % 10)
	var seq [4]int8
	seqSum := int8(0)

	for i := 1; i <= rounds; i++ {
		currNumber := currSecretNumber(lastNumber)
		currPrice := uint8(currNumber % 10)
		change := int8(currPrice) - int8(lastPrice)
		seq[3] = change
		seqSum += change
		seqI := sequenceIndex(seq)

		// ignore negative sequences, as they are incredibly unlikely to lead to a best outcome
		if i > 3 && seqSum >= 0 {
			// add to sequences
			if !seen[seqI] {
				// only record the first time each sequence is seen
				seen[seqI] = true
				combined[seqI] += uint16(currPrice)
			}
		}

		lastNumber = currNumber
		lastPrice = currPrice
		seqSum -= seq[0]
		copy(seq[0:3], seq[1:4])
	}
}

func part1(initial []uint32) int {
	return lo.SumBy(initial, func(n uint32) int { return int(nthSecretNumber(n, 2000)) })
}

func part2(initial []uint32) int {
	combined := [sequencesSize]uint16{}
	seen := [sequencesSize]bool{}

	for _, n := range initial {
		clear(seen[:])
		calculatePriceChangeSequences(n, 2000, &combined, &seen)
	}

	res := uint16(0)
	for _, bananas := range combined {
		res = max(res, bananas)
	}
	return int(res)
}
