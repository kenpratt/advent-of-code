package day23

import (
	"adventofcode/set"
	"adventofcode/util"
	"slices"
	"strings"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1330, part1(&input))
	util.AssertEqual(0, part2(&input))
}

type Input struct {
	connections [][]int
	nodes       map[int]string
}

func parseInput(input string) Input {
	lines := strings.Split(input, "\n")

	ids := make(map[string]int, len(lines))

	// parse the pairs and swap the strings with ints, for efficiency
	pairs := lo.Map(lines, func(s string, _ int) [2]int {
		parts := strings.Split(s, "-")
		util.AssertEqual(2, len(parts))
		a := getId(parts[0], ids)
		b := getId(parts[1], ids)
		return [2]int{a, b}
	})

	// reverse lookup table from id->name
	names := make(map[int]string, len(ids))
	for s, id := range ids {
		names[id] = s
	}

	// build up a lookup table of connections
	connections := make([][]int, len(ids))
	for _, pair := range pairs {
		a, b := pair[0], pair[1]
		// record both directions
		connections[a] = append(connections[a], b)
		connections[b] = append(connections[b], a)
	}

	return Input{connections, names}
}

func getId(s string, ids map[string]int) int {
	if id, ok := ids[s]; ok {
		return id
	} else {
		id = len(ids)
		ids[s] = id
		return id
	}
}

func findGroupsOf3(initial []int, input *Input) int {
	groups := set.NewSet[[3]int]()

	for _, a := range initial {
		for _, b := range input.connections[a] {
			for _, c := range input.connections[b] {
				// see if c is connected back to a
				if a != c && lo.Contains(input.connections[c], a) {
					group := [3]int{a, b, c}
					slices.Sort(group[:])
					groups.Add(group)
				}
			}
		}
	}

	return groups.Len()
}

func part1(input *Input) int {
	includeIds := make([]int, 0)

	// find nodes starting with t
	for id, s := range input.nodes {
		if s[0] == 't' {
			includeIds = append(includeIds, id)
		}
	}

	return findGroupsOf3(includeIds, input)
}

func part2(input *Input) int {
	return 0
}
