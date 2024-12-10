package day02

import (
	"adventofcode/util"
	"strings"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	util.AssertEqual(510, part1(input))
	util.AssertEqual(553, part2(input))
}

func parseInput(input string) [][]int {
	lines := strings.Split(input, "\n")

	reports := make([][]int, len(lines))

	for i, line := range lines {
		parts := strings.Fields(line)

		reports[i] = make([]int, len(parts))
		for j, part := range parts {
			reports[i][j] = util.StringToInt(part)
		}

	}

	return reports
}

func part1(input string) int {
	reports := parseInput(input)

	safe := 0
	for _, report := range reports {
		if isSafe(report, -1) {
			safe++
		}
	}
	return safe
}

func part2(input string) int {
	reports := parseInput(input)

	safe := 0
	for _, report := range reports {
		if isSafe(report, -1) {
			safe++
		} else {
			// try removing each level
			for i := 0; i < len(report); i++ {
				if isSafe(report, i) {
					safe++
					break
				}
			}
		}
	}
	return safe
}

func isSafe(report []int, skipIndex int) bool {
	increasing := true
	increasingInitialized := false

	for i := 0; i < len(report)-1; i++ {
		if i == skipIndex {
			continue
		}

		j := i + 1
		if j == skipIndex {
			j++
		}
		if j >= len(report) {
			continue
		}

		if !increasingInitialized {
			increasing = report[j] > report[i]
			increasingInitialized = true
		}

		if (increasing && report[j] <= report[i]) || (!increasing && report[j] >= report[i]) {
			return false
		}

		d := util.AbsDiff(report[i], report[j])
		if d < 1 || d > 3 {
			return false
		}
	}
	return true
}
