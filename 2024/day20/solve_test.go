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
	expected := 0
	input := parseInput(example)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 0
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
