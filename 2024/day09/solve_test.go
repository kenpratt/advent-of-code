package day09

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = "2333133121414131402"

func TestPart1Example(t *testing.T) {
	expected := 1928
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 6332189866718
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 2858
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 6353648390778
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
