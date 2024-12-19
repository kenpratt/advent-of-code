package day18

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0`

func TestPart1Example(t *testing.T) {
	expected := 22
	input := parseInput(example, 7)
	actual := part1(input, 12)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 272
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := "6,1"
	input := parseInput(example, 7)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := "16,44"
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
