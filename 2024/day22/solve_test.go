package day22

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example1 = `1
10
100
2024`

const example2 = `1
2
3
2024`

var expectedFor123 = [10]uint32{
	15887950,
	16495136,
	527345,
	704524,
	1553684,
	12683156,
	11100544,
	12249484,
	7753432,
	5908254,
}

func TestNthSecretNumber(t *testing.T) {
	for i, expected := range expectedFor123 {
		assert.Equal(t, expected, nthSecretNumber(123, i+1))
	}
}

func TestPart1Example(t *testing.T) {
	expected := 37327623
	input := parseInput(example1)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 14273043166
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestCalculatePricesAndChanges(t *testing.T) {
	expected := map[[4]int8]uint16{
		{-3, 6, -1, -1}: 4,
		{-1, -1, 0, 2}:  6,
		{0, 2, -2, 0}:   4,
		{6, -1, -1, 0}:  4,
	}
	combined := [sequencesSize]uint16{}
	seen := [sequencesSize]bool{}
	calculatePriceChangeSequences(123, 10, &combined, &seen)

	expectedSum := uint16(0)
	for seq, n := range expected {
		seqI := sequenceIndex(seq)
		assert.Equal(t, n, combined[seqI])
		expectedSum += n
	}

	actualSum := uint16(0)
	for _, v := range combined {
		actualSum += v
	}
	assert.Equal(t, expectedSum, actualSum)
}

func TestPart2Example(t *testing.T) {
	expected := 23
	input := parseInput(example2)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 1667
	input := parseInput(util.ReadInputFile("."))
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		input := parseInput(util.ReadInputFile("."))
		part1(input)
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		input := parseInput(util.ReadInputFile("."))
		part2(input)
	}
}
