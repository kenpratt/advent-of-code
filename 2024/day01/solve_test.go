package day01

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `3   4
4   3
2   5
1   3
3   9
3   3`

func TestPart1Example(t *testing.T) {
	expected := 11
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 2264607
	actual := part1(readInputFile())
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 31
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 19457120
	actual := part2(readInputFile())
	assert.Equal(t, expected, actual)
}
