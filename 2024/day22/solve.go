package day22

import (
	"adventofcode/util"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	finalSecrets, bananasForSequnce := parseInput(inputStr)
	util.AssertEqual(14273043166, part1(&finalSecrets))
	util.AssertEqual(1667, part2(&bananasForSequnce))
}

func parseInput(input string) ([]uint32, [sequencesSize]uint16) {
	lines := strings.Split(input, "\n")
	initial := lo.Map(lines, func(s string, _ int) uint32 { return uint32(util.StringToInt(s)) })
	return calculate(initial)
}

func nthSecretNumber(initial uint32, nth int) uint32 {
	prev := initial
	for i := 0; i < nth; i++ {
		curr := currSecretNumber(prev)
		prev = curr
	}
	return prev
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

func calculate(initial []uint32) ([]uint32, [sequencesSize]uint16) {
	finalSecrets := make([]uint32, len(initial))
	bananasForSequnce := [sequencesSize]uint16{}
	seen := [sequencesSize]bool{}

	for i, n := range initial {
		clear(seen[:])
		finalSecrets[i] = calculateNumber(n, 2000, &bananasForSequnce, &seen)
	}

	return finalSecrets, bananasForSequnce
}

func calculateNumber(initial uint32, rounds int, bananasForSequnce *[sequencesSize]uint16, seen *[sequencesSize]bool) uint32 {
	prevNumber := initial
	prevPrice := uint8(initial % 10)

	// use rolling values for sequence of four changes
	// - seqSum is a sum of the previous 4 changes
	// - seqI in an index for the combined/seen arrays, and it is the previous 4 changes combined into a uint32, as a base 20 number
	seqSum := uint8(0)
	seqI := uint32(0)

	for i := 1; i <= rounds; i++ {
		currNumber := currSecretNumber(prevNumber)
		currPrice := uint8(currNumber % 10)
		change := 10 + currPrice - prevPrice // add 10 so it's between 1-19 instead of -9 to 9
		seqSum += change
		seqI += uint32(change)

		// add to sequences
		// ignore negative sequences, as they are incredibly unlikely to lead to a best outcome
		// a seq of 0,0,0,0 will be 40 due to adding 10 to it
		if i > 3 && seqSum >= 40 && !seen[seqI] {
			// only record the first time each sequence is seen
			seen[seqI] = true
			bananasForSequnce[seqI] += uint16(currPrice)
		}

		prevNumber = currNumber
		prevPrice = currPrice

		seq0 := seqI / 8000
		seqSum -= uint8(seq0)

		// get seqI ready for next round by removing the oldest value and shoving the other values over by multiplying by 20
		seqI = (seqI - uint32(seq0)*8000) * 20
	}

	return prevNumber
}

func part1(finalSecrets *[]uint32) int {
	sum := 0
	for _, v := range *finalSecrets {
		sum += int(v)
	}
	return sum
}

func part2(bananasForSequnce *[sequencesSize]uint16) int {
	res := uint16(0)
	for _, bananas := range bananasForSequnce {
		res = max(res, bananas)
	}
	return int(res)
}
