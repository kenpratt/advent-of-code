package day01

import "testing"

const example1 = `3   4
4   3
2   5
1   3
3   9
3   3`

func TestPart1Example(t *testing.T) {
	expected := 11
	actual := part1(example1)
	assertEqual(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 2264607
	actual := part1(readInputFile())
	assertEqual(t, expected, actual)
}

func assertEqual(t *testing.T, expected int, actual int) {
	if expected != actual {
		t.Errorf("expected %d; actual %d", expected, actual)
	}
}
