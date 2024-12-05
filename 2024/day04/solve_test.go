package day04

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`

func TestPart1Example(t *testing.T) {
	expected := 18
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 2662
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 9
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 2034
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
