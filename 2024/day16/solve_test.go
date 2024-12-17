package day16

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example1 = `###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############`

const example2 = `#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################`

func TestPart1Example1(t *testing.T) {
	expected := 7036
	input := parseInput(example1)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Example2(t *testing.T) {
	expected := 11048
	input := parseInput(example2)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 88416
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example1(t *testing.T) {
	expected := 45
	input := parseInput(example1)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example2(t *testing.T) {
	expected := 64
	input := parseInput(example2)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 442
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
