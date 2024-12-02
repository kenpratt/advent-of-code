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
	fmt.Println("part 2: ", part2(input))
}

type List struct {
	left  []int
	right []int
}

func parseInput(input string) List {
	lines := strings.Split(input, "\n")

	left := make([]int, len(lines))
	right := make([]int, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			panic(fmt.Sprintf("line should have 2 parts but has %d: %s", len(parts), parts))
		}

		left[i] = stringToInt(parts[0])
		right[i] = stringToInt(parts[1])
	}

	return List{left, right}
}

func part1(input string) int {
	list := parseInput(input)

	// sort the lists
	sort.Ints(list.left)
	sort.Ints(list.right)

	result := 0
	for i := range list.left {
		result += absInt(list.left[i] - list.right[i])
	}
	return result
}

func part2(input string) int {
	list := parseInput(input)

	rightCounts := make(map[int]int)
	for _, v := range list.right {
		rightCounts[v]++
	}

	result := 0
	for _, v := range list.left {
		result += v * rightCounts[v]
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
