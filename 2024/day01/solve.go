package day01

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func Solve() {
	input := readInputFile()
	fmt.Println("part 1: ", part1(input))
}

func part1(input string) int {
	lines := strings.Split(input, "\n")

	first := make([]int, len(lines))
	second := make([]int, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			panic(fmt.Sprintf("line should have 2 parts but has %d: %s", len(parts), parts))
		}

		first[i] = stringToInt(parts[0])
		second[i] = stringToInt(parts[1])
	}

	// sort them
	sort.Ints(first)
	sort.Ints(second)

	result := 0
	for i := range first {
		result += absInt(first[i] - second[i])
	}

	return result
}

func readInputFile() string {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	return string(input)
}

func stringToInt(s string) int {
	v, err := strconv.Atoi(s)
	if err != nil {
		// ... handle error
		panic(err)
	}
	return v
}

func absInt(x int) int {
	if x >= 0 {
		return x
	} else {
		return -x
	}
}
