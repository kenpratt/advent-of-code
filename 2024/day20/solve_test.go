package day20

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############`

func TestPart1Example(t *testing.T) {
	input := parseInput(example)
	assert.Equal(t, 0, part1(input, 65))
	assert.Equal(t, 1, part1(input, 64))
	assert.Equal(t, 2, part1(input, 40))
	assert.Equal(t, 3, part1(input, 38))
	assert.Equal(t, 4, part1(input, 36))
	assert.Equal(t, 5, part1(input, 20))
	assert.Equal(t, 8, part1(input, 12))
	assert.Equal(t, 10, part1(input, 10))
	assert.Equal(t, 14, part1(input, 8))
	assert.Equal(t, 16, part1(input, 6))
	assert.Equal(t, 30, part1(input, 4))
	assert.Equal(t, 44, part1(input, 2))
	assert.Equal(t, 44, part1(input, 0))
}

func TestPart1Input(t *testing.T) {
	expected := 1518
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	input := parseInput(example)
	assert.Equal(t, 0, part2(input, 77))
	assert.Equal(t, 3, part2(input, 76))
	assert.Equal(t, 7, part2(input, 74))
	assert.Equal(t, 29, part2(input, 72))
	assert.Equal(t, 41, part2(input, 70))
	assert.Equal(t, 55, part2(input, 68))
	assert.Equal(t, 67, part2(input, 66))
	assert.Equal(t, 86, part2(input, 64))
	assert.Equal(t, 106, part2(input, 62))
	assert.Equal(t, 129, part2(input, 60))
	assert.Equal(t, 154, part2(input, 58))
	assert.Equal(t, 193, part2(input, 56))
	assert.Equal(t, 222, part2(input, 54))
	assert.Equal(t, 253, part2(input, 52))
	assert.Equal(t, 285, part2(input, 50))
}

func TestPart2Input(t *testing.T) {
	expected := 1032257
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
