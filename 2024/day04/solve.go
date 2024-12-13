package day04

import (
	"adventofcode/grid"
	"adventofcode/util"
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
	xCoords := make([]grid.Coord, 0)
	aCoords := make([]grid.Coord, 0)

	g := grid.Parse(input, func(c rune, pos grid.Coord) rune {
		if c == 'X' {
			xCoords = append(xCoords, pos)
		} else if c == 'A' {
			aCoords = append(aCoords, pos)
		}

		return c
	})

	return WordSearch{grid: g, xCoords: xCoords, aCoords: aCoords}
}

func part1(ws WordSearch) int {
	var directions = grid.DiagonalOffsets()

	result := 0
	for _, x := range ws.xCoords {
		for _, offset := range directions {
			if m, ok := ws.grid.AddOffset(x, offset); ok && ws.grid.At(m) == 'M' {
				if a, ok := ws.grid.AddOffset(m, offset); ok && ws.grid.At(a) == 'A' {
					if s, ok := ws.grid.AddOffset(a, offset); ok && ws.grid.At(s) == 'S' {
						result++
					}
				}
			}
		}
	}
	return result
}

func part2(ws WordSearch) int {
	ulo := grid.MakeOffset(-1, -1)
	uro := grid.MakeOffset(1, -1)
	dlo := grid.MakeOffset(-1, 1)
	dro := grid.MakeOffset(1, 1)

	result := 0
	for _, a := range ws.aCoords {
		var ul, ur, dl, dr rune

		if ulc, ok := ws.grid.AddOffset(a, ulo); ok {
			ul = ws.grid.At(ulc)
		}
		if urc, ok := ws.grid.AddOffset(a, uro); ok {
			ur = ws.grid.At(urc)
		}
		if dlc, ok := ws.grid.AddOffset(a, dlo); ok {
			dl = ws.grid.At(dlc)
		}
		if drc, ok := ws.grid.AddOffset(a, dro); ok {
			dr = ws.grid.At(drc)
		}

		if ((ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M')) && ((ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M')) {
			result++
		}
	}
	return result
}
