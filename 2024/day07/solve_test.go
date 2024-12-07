package day07

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`

func TestPart1Example(t *testing.T) {
	expected := 3749
	actual := part1(example)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 2501605301465
	actual := part1(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 11387
	actual := part2(example)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 44841372855953
	actual := part2(util.ReadInputFile("."))
	assert.Equal(t, expected, actual)
}
