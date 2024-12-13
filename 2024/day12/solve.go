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

func parseInput(input string) []RegionMetadata {
	plots := grid.Parse(input, func(c rune, _ grid.Coord) rune {
		return c
	})
	solver := MakeSolver(plots)
	return solver.GenerateMetadata()
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

type RegionMetadata struct {
	area      int
	perimeter int
	sides     int
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

func (solver *Solver) GenerateMetadata() []RegionMetadata {
	bounds := solver.plots.Bounds
	result := make([]RegionMetadata, 0)
	toExpand := stack.NewStack[grid.Coord](8)

	// visit each plot
	for pos, kind := range solver.plots.Iter() {
		if solver.explored.At(pos) {
			// we've already explored this one, skip it
			continue
		}
		solver.explored.Set(pos, true)

		// start a new region
		region := MakeRegion(kind, pos, &solver.plots, &toExpand)

		// fully expand the region
		for toExpand.Len() > 0 {
			expand := toExpand.Pop()
			if !solver.explored.At(expand) {
				solver.explored.Set(expand, true)
				region.Add(expand, &solver.plots, &toExpand)
			}
		}

		// set the perimeter
		region.CalculatePerimeter(bounds)

		// add the region
		result = append(result, region.CalculateMetadata(bounds))
	}

	return result
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
	if posKind := plots.At(pos); region.kind != posKind {
		panic("Trying to add a point to region of a mismatched kind")
	}

	region.area.Add(pos)
	for n := range plots.IterNeighbours(pos) {
		nKind := plots.At(n)
		if region.kind == nKind {
			toExpand.Push(n)
		}
	}
}

func (region *Region) CalculatePerimeter(bounds grid.Bounds) {
	edges := set.NewSet[Edge]()

	for pos := range region.area.Iter() {
		for _, d := range grid.Directions() {
			n, inBounds := bounds.MoveInDirection(pos, d, 1)
			if !(inBounds && region.area.Contains(n)) {
				edges.Add(Edge{pos, d})
			}
		}
	}

	region.perimeter = edges
}

func (edge Edge) NeighbourBefore(bounds grid.Bounds) (Edge, bool) {
	switch edge.facing {
	case grid.North, grid.South:
		// vertical edge
		return edge.MoveInDirection(grid.West, bounds)
	case grid.West, grid.East:
		// horizontal edge
		return edge.MoveInDirection(grid.North, bounds)
	default:
		panic("Unreachable")
	}
}

func (edge Edge) NeighbourAfter(bounds grid.Bounds) (Edge, bool) {
	switch edge.facing {
	case grid.North, grid.South:
		// vertical edge
		return edge.MoveInDirection(grid.East, bounds)
	case grid.West, grid.East:
		// horizontal edge
		return edge.MoveInDirection(grid.South, bounds)
	default:
		panic("Unreachable")
	}
}

func (edge Edge) MoveInDirection(d grid.Direction, bounds grid.Bounds) (Edge, bool) {
	pos, inBounds := bounds.MoveInDirection(edge.pos, d, 1)
	if inBounds {
		return Edge{
			pos,
			edge.facing,
		}, true
	} else {
		return Edge{
			-1,
			0,
		}, false
	}
}

func (region *Region) NumSides(bounds grid.Bounds) int {
	perimeter := region.perimeter

	neighbourCount := 0
	for edge := range region.perimeter.Iter() {
		if n, ok := edge.NeighbourBefore(bounds); ok && perimeter.Contains(n) {
			neighbourCount++
		}
		if n, ok := edge.NeighbourAfter(bounds); ok && perimeter.Contains(n) {
			neighbourCount++
		}
	}

	// number of sides is equal to perimeter size minus the adjacent edges
	// neighbours are counted twice, once from each side, so divide by 2
	return perimeter.Len() - neighbourCount/2
}

func (region *Region) CalculateMetadata(bounds grid.Bounds) RegionMetadata {
	return RegionMetadata{
		area:      region.area.Len(),
		perimeter: region.perimeter.Len(),
		sides:     region.NumSides(bounds),
	}
}

func (region *RegionMetadata) Price() int {
	return region.area * region.perimeter
}

func (region *RegionMetadata) BulkPrice() int {
	return region.area * region.sides
}

func part1(regions []RegionMetadata) int {
	return lo.SumBy(regions, func(r RegionMetadata) int { return r.Price() })
}

func part2(regions []RegionMetadata) int {
	return lo.SumBy(regions, func(r RegionMetadata) int { return r.BulkPrice() })
}
