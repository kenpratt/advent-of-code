package day03

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
const example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

func TestPart1Example(t *testing.T) {
	expected := 161
	actual := part1(example1)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 187194524
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 48
	actual := part2(example2)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 127092535
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
