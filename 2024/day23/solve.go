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
	util.AssertEqual("hl,io,ku,pk,ps,qq,sh,tx,ty,wq,xi,xj,yp", part2(&input))
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

type BitField struct {
	// hard-coded so that it can be used as a map key
	// 9 blocks to support inputs up to 576 bits wide
	blocks [9]uint64
	numSet int
}

func MakeBitField(connections []int) BitField {
	field := BitField{}
	for _, to := range connections {
		field.Set(to)
	}
	return field
}

func (c *BitField) Clone() BitField {
	field := BitField{}
	copy(field.blocks[:], c.blocks[:])
	field.numSet = c.numSet
	return field
}

func (c *BitField) IsSet(id int) bool {
	i := id / 64
	v := id % 64
	return c.blocks[i]>>v&1 == 1
}

func (c *BitField) Set(id int) {
	i := id / 64
	v := id % 64
	c.blocks[i] |= 1 << v
	c.numSet++ // TODO what if id is already set?
}

func (c *BitField) Unset(id int) {
	i := id / 64
	v := id % 64
	c.blocks[i] &= ^(1 << v)
	c.numSet-- // TODO what if id is already unset?
}

func (c *BitField) toNames(input *Input) []string {
	names := make([]string, 0, c.numSet)
	for i := 0; i < 9*64; i++ {
		if c.IsSet(i) {
			names = append(names, input.nodes[i])
		}
	}
	slices.Sort(names)
	return names
}

func findAllGroupsOf3(initial []int, input *Input) int {
	groups := set.NewSet[[3]int]()

	for _, a := range initial {
		for _, b := range input.connections[a] {
			for _, c := range input.connections[b] {
				// see if c is BitField back to a
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

func findLargestConnectedGroup(input *Input) []string {
	// find the largest number of connections, for a starting size
	mostConnections := 0
	for _, conns := range input.connections {
		mostConnections = max(mostConnections, len(conns))
	}

	for tryCount := mostConnections; tryCount > 0; tryCount-- {
		if group, ok := findGroupOfSize(tryCount, input); ok {
			return group
		}
	}

	panic("no solution found")
}

func findGroupOfSize(size int, input *Input) ([]string, bool) {
	counts := make(map[BitField]int, 0)

	// for each list of connections, find all combinations of this size
	for id, connections := range input.connections {
		field := MakeBitField(connections)
		field.Set(id)

		if field.numSet < size {
			// skip, not enough connections
			continue
		} else if field.numSet == size {
			// simple case, exact right number
			counts[field]++
		} else if field.numSet-size == 1 {
			// remove one connection
			for _, rem := range connections {
				field2 := field.Clone()
				field2.Unset(rem)
				counts[field2]++
				if counts[field2] == size {
					return field2.toNames(input), true
				}
			}
		} else {
			panic("Implement group of size smaller than 1 under number of connections")
		}
	}

	return []string{}, false
}

func part1(input *Input) int {
	includeIds := make([]int, 0)

	// find nodes starting with t
	for id, s := range input.nodes {
		if s[0] == 't' {
			includeIds = append(includeIds, id)
		}
	}

	return findAllGroupsOf3(includeIds, input)
}

func part2(input *Input) string {
	group := findLargestConnectedGroup(input)
	return strings.Join(group, ",")
}
