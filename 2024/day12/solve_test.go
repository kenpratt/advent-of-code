package day12

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example1 = `AAAA
BBCD
BBCC
EEEC`

const example2 = `OOOOO
OXOXO
OOOOO
OXOXO
OOOOO`

const example3 = `RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`

const example4 = `EEEEE
EXXXX
EEEEE
EXXXX
EEEEE`

const example5 = `AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA`

func TestPart1Example1(t *testing.T) {
	expected := 140
	input := parseInput(example1)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Example2(t *testing.T) {
	expected := 772
	input := parseInput(example2)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Example3(t *testing.T) {
	expected := 1930
	input := parseInput(example3)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 1452678
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example1(t *testing.T) {
	expected := 80
	input := parseInput(example1)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example2(t *testing.T) {
	expected := 436
	input := parseInput(example2)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example3(t *testing.T) {
	expected := 1206
	input := parseInput(example3)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example4(t *testing.T) {
	expected := 236
	input := parseInput(example4)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example5(t *testing.T) {
	expected := 368
	input := parseInput(example5)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 873584
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
