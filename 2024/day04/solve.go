package day04

import (
	"adventofcode/grid"
	"adventofcode/util"
	"strings"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(2662, part1(input))
	util.AssertEqual(2034, part2(input))
}

type WordSearch struct {
	grid    grid.Grid[rune]
	xCoords []grid.Coord
	aCoords []grid.Coord
}

func parseInput(input string) WordSearch {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := grid.Bounds{Width: width, Height: height}
	values := make([]rune, width*height)
	xCoords := make([]grid.Coord, 0)
	aCoords := make([]grid.Coord, 0)

	for y, line := range lines {
		for x, char := range line {
			pos := grid.MakeCoord(x, y)

			if char == 'X' {
				xCoords = append(xCoords, pos)
			} else if char == 'A' {
				aCoords = append(aCoords, pos)
			}

			values[bounds.CoordToIndex(&pos)] = char
		}
	}

	g := grid.Grid[rune]{Bounds: bounds, Values: values}

	return WordSearch{grid: g, xCoords: xCoords, aCoords: aCoords}
}

func part1(ws WordSearch) int {
	var directions = grid.DiagonalOffsets()

	result := 0
	for _, x := range ws.xCoords {
		for _, offset := range directions {
			m := x.Add(&offset)
			if c, f := ws.grid.At(&m); f && *c == 'M' {
				a := m.Add(&offset)
				if c, f := ws.grid.At(&a); f && *c == 'A' {
					s := a.Add(&offset)
					if c, f := ws.grid.At(&s); f && *c == 'S' {
						result++
					}
				}
			}
		}
	}
	return result
}

func part2(ws WordSearch) int {
	ulo := grid.MakeCoord(-1, -1)
	uro := grid.MakeCoord(1, -1)
	dlo := grid.MakeCoord(-1, 1)
	dro := grid.MakeCoord(1, 1)

	result := 0
	for _, a := range ws.aCoords {
		ulc := a.Add(&ulo)
		urc := a.Add(&uro)
		dlc := a.Add(&dlo)
		drc := a.Add(&dro)

		ul, _ := ws.grid.At(&ulc)
		ur, _ := ws.grid.At(&urc)
		dl, _ := ws.grid.At(&dlc)
		dr, _ := ws.grid.At(&drc)

		if ((*ul == 'M' && *dr == 'S') || (*ul == 'S' && *dr == 'M')) && ((*ur == 'M' && *dl == 'S') || (*ur == 'S' && *dl == 'M')) {
			result++
		}
	}
	return result
}
