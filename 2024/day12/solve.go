package day12

import (
	"adventofcode/grid"
	"adventofcode/set"
	"adventofcode/stack"
	"adventofcode/util"

	"github.com/samber/lo"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(1452678, part1(input))
	util.AssertEqual(873584, part2(input))
}

func parseInput(input string) []Region {
	plots := grid.Parse(input, func(c rune, _ grid.Coord) rune {
		return c
	})
	solver := MakeSolver(plots)
	return solver.Regions()
}

type Solver struct {
	plots    grid.Grid[rune]
	explored grid.Grid[bool]
}

type Region struct {
	kind      rune
	area      set.Set[grid.Coord]
	perimeter set.Set[Edge]
}

type Edge struct {
	pos    grid.Coord
	facing grid.Direction
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

func (solver *Solver) Regions() []Region {
	regions := make([]Region, 0)
	toExpand := stack.NewStack[grid.Coord](8)

	// visit each plot
	for i, kind := range solver.plots.Values {
		if solver.explored.Values[i] {
			// we've already explored this one, skip it
			continue
		}
		solver.explored.Values[i] = true

		// start a new region
		pos := solver.plots.Bounds.IndexToCoord(i)
		region := MakeRegion(kind, pos, &solver.plots, &toExpand)

		// fully expand the region
		for toExpand.Len() > 0 {
			expand := toExpand.Pop()
			expandi := solver.plots.Bounds.CoordToIndex(expand)
			if !solver.explored.Values[expandi] {
				solver.explored.Values[expandi] = true
				region.Add(expand, &solver.plots, &toExpand)
			}
		}

		// set the perimeter
		region.CalculatePerimeter()

		// add the region
		regions = append(regions, region)
	}

	return regions
}

func MakeRegion(kind rune, pos grid.Coord, plots *grid.Grid[rune], toExpand *stack.Stack[grid.Coord]) Region {
	region := Region{
		kind: kind,
		area: set.NewSet[grid.Coord](),
		// leave perimeter uninitialized, it will be set later
	}
	region.Add(pos, plots, toExpand)
	return region
}

func (region *Region) Add(pos grid.Coord, plots *grid.Grid[rune], toExpand *stack.Stack[grid.Coord]) {
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
			toExpand.Push(n)
		}
	}
}

func (region *Region) CalculatePerimeter() {
	edges := set.NewSet[Edge]()

	for pos := range region.area.Iter() {
		for _, d := range grid.Directions() {
			n := pos.MoveInDirection(d, 1)
			if !region.area.Contains(n) {
				edges.Add(Edge{n, d})
			}
		}
	}

	region.perimeter = edges
}

func (edge Edge) Neighbours() [2]Edge {
	switch edge.facing {
	case grid.North, grid.South:
		// vertical edge, check left and right
		left := edge.MoveInDirection(grid.West)
		right := edge.MoveInDirection(grid.East)
		return [2]Edge{left, right}
	case grid.West, grid.East:
		// horizontal edge, check above and below
		above := edge.MoveInDirection(grid.North)
		below := edge.MoveInDirection(grid.South)
		return [2]Edge{above, below}
	default:
		panic("Unreachable")
	}
}

func (edge Edge) MoveInDirection(d grid.Direction) Edge {
	return Edge{
		edge.pos.MoveInDirection(d, 1),
		edge.facing,
	}
}

func (region *Region) NumSides() int {
	perimeter := region.perimeter

	neighbourCount := 0
	for edge := range region.perimeter.Iter() {
		for _, n := range edge.Neighbours() {
			if perimeter.Contains(n) {
				neighbourCount++
			}
		}
	}

	// number of sides is equal to perimeter size minus the adjacent edges
	// neighbours are counted twice, once from each side, so divide by 2
	return perimeter.Len() - neighbourCount/2
}

func (region *Region) Price() int {
	area := region.area.Len()
	perimeter := region.perimeter.Len()
	price := area * perimeter
	return price
}

func (region *Region) BulkPrice() int {
	area := region.area.Len()
	sides := region.NumSides()
	price := area * sides
	return price
}

func part1(regions []Region) int {
	return lo.SumBy(regions, func(region Region) int { return region.Price() })
}

func part2(regions []Region) int {
	return lo.SumBy(regions, func(region Region) int { return region.BulkPrice() })
}
