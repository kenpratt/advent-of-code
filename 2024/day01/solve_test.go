package day01

import "testing"

const example = `3   4
4   3
2   5
1   3
3   9
3   3`

func TestPart1Example(t *testing.T) {
	expected := 11
	actual := part1(example)
	assertEqual(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 2264607
	actual := part1(readInputFile())
	assertEqual(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := 31
	actual := part2(example)
	assertEqual(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := 19457120
	actual := part2(readInputFile())
	assertEqual(t, expected, actual)
}

func assertEqual(t *testing.T, expected int, actual int) {
	if expected != actual {
		t.Errorf("expected %d; actual %d", expected, actual)
	}
}
