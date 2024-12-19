package day17

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example1 = `Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0`

const example2 = `Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0`

func TestInstructionExample1(t *testing.T) {
	input := Input{0, 0, 9, []uint8{2, 6}}
	comp := MakeSimpleComputer(input)
	comp.Run()
	assert.Equal(t, "", comp.OutputString())
	assert.Equal(t, 1, comp.b)
}

func TestInstructionExample2(t *testing.T) {
	input := Input{10, 0, 0, []uint8{5, 0, 5, 1, 5, 4}}
	comp := MakeSimpleComputer(input)
	comp.Run()
	assert.Equal(t, "0,1,2", comp.OutputString())
}

func TestInstructionExample3(t *testing.T) {
	input := Input{2024, 0, 0, []uint8{0, 1, 5, 4, 3, 0}}
	comp := MakeSimpleComputer(input)
	comp.Run()
	assert.Equal(t, "4,2,5,6,7,7,7,7,3,1,0", comp.OutputString())
	assert.Equal(t, 0, comp.a)
}

func TestInstructionExample4(t *testing.T) {
	input := Input{0, 29, 0, []uint8{1, 7}}
	comp := MakeSimpleComputer(input)
	comp.Run()
	assert.Equal(t, "", comp.OutputString())
	assert.Equal(t, 26, comp.b)
}

func TestInstructionExample5(t *testing.T) {
	input := Input{0, 2024, 43690, []uint8{4, 0}}
	comp := MakeSimpleComputer(input)
	comp.Run()
	assert.Equal(t, "", comp.OutputString())
	assert.Equal(t, 44354, comp.b)
}

func TestPart1Example(t *testing.T) {
	expected := "4,6,3,5,6,3,5,2,1,0"
	input := parseInput(example1)
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := "5,0,3,5,7,6,1,5,4"
	input := parseInput(util.ReadInputFile("."))
	actual := part1(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 117440
	input := parseInput(example2)
	actual := part2(input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 164516454365621
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
