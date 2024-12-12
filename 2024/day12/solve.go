package day12

import (
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/stack"
	"adventofcode/util"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1452678, part1(input))
	util.AssertEqual(0, part2(input))
}

func parseInput(input string) grid.Grid[rune] {
	return grid.Parse(input, func(c rune, _ grid.Coord) rune {
		return c
	})
}

type Solver struct {
	plots    grid.Grid[rune]
	explored grid.Grid[bool]
}

type Region struct {
	kind     rune
	area     set.Set[grid.Coord]
	toExpand stack.Stack[grid.Coord]
}

type Edge struct {
	a grid.Coord
	b grid.Coord
}

func MakeSolver(plots grid.Grid[rune]) Solver {
	explored := grid.Grid[bool]{
		Bounds: plots.Bounds,
		Values: make([]bool, plots.Len()),
	}
	return Solver{
		plots, explored,
	}
}

func MakeRegion(kind rune, pos grid.Coord, plots *grid.Grid[rune]) Region {
	region := Region{kind: kind, area: set.NewSet[grid.Coord](), toExpand: stack.NewStack[grid.Coord](8)}
	region.Add(pos, plots)
	return region
}

func (region *Region) Add(pos grid.Coord, plots *grid.Grid[rune]) {
	if region.area.Contains(pos) {
		return
	}
	if posKind, _ := plots.At(pos); region.kind != posKind {
		panic("Trying to add a point to region of a mismatched kind")
	}

	region.area.Add(pos)
	for n := range plots.IterNeighbours(pos) {
		nKind, _ := plots.At(n)
		if region.kind == nKind {
			// TODO try recursive?
			region.toExpand.Push(n)
		}
	}
}

func (region *Region) Area() int {
	return region.area.Len()
}

func (region *Region) Perimeter() int {
	edgeCounts := make(map[Edge]int)

	for pos := range region.area.Iter() {
		for _, d := range grid.Directions() {
			n := pos.MoveInDirection(d, 1)
			switch d {
			case grid.South, grid.East:
				edgeCounts[Edge{pos, n}]++
			case grid.North, grid.West:
				edgeCounts[Edge{n, pos}]++
			}
		}
	}

	// any shared edge will have an edge count > 1
	result := 0
	for _, n := range edgeCounts {
		if n == 1 {
			result++
		}
	}
	return result
}

func (region *Region) Price() int {
	area := region.Area()
	perimeter := region.Perimeter()
	price := area * perimeter
	return price
}

func (solver *Solver) TotalPrice() int {
	result := 0

	// visit each plot
	for i, kind := range solver.plots.Values {
		if solver.explored.Values[i] {
			// we've already explored this one, skip it
			continue
		}
		solver.explored.Values[i] = true

		// start a new region
		pos := solver.plots.Bounds.IndexToCoord(i)
		region := MakeRegion(kind, pos, &solver.plots)

		// fully expand the region
		for region.toExpand.Len() > 0 {
			expand := region.toExpand.Pop()
			expandi := solver.plots.Bounds.CoordToIndex(expand)
			if !solver.explored.Values[expandi] {
				solver.explored.Values[expandi] = true
				region.Add(expand, &solver.plots)
			}
		}

		// add the cost of the region
		result += region.Price()
	}

	return result
}

func part1(plots grid.Grid[rune]) int {
	solver := MakeSolver(plots)
	return solver.TotalPrice()
}

func part2(_ grid.Grid[rune]) int {
	return 0
}
