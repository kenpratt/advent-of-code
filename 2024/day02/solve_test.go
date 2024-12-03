package day02

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

func TestPart1Example(t *testing.T) {
	expected := 2
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 510
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 4
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 553
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
