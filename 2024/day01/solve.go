package day01

import (
	"adventofcode/util"
	"fmt"
	"sort"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(2264607, part1(input))
	util.AssertEqual(19457120, part2(input))
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
		parts := strings.Fields(line)
		if len(parts) != 2 {
			panic(fmt.Sprintf("line should have 2 parts but has %d: %s", len(parts), parts))
		}

		left[i] = util.StringToInt(parts[0])
		right[i] = util.StringToInt(parts[1])
	}

	return List{left, right}
}

func part1(list List) int {
	// sort the lists
	sort.Ints(list.left)
	sort.Ints(list.right)

	result := 0
	for i := range list.left {
		result += util.AbsInt(list.left[i] - list.right[i])
	}
	return result
}

func part2(list List) int {
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
