package day06

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`

func TestPart1Example(t *testing.T) {
	expected := 41
	input := parseInput(example)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 4647
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 6
	input := parseInput(example)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 1723
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
