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
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 4647
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 6
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 1723
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
