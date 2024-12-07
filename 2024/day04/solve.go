package day04

import (
	"adventofcode/grid"
	"adventofcode/util"
	"fmt"
	"strings"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	fmt.Println("part 1: ", part1(input))
	fmt.Println("part 2: ", part2(input))
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

			values[grid.CoordToIndex(&bounds, &pos)] = char
		}
	}

	grid := grid.Grid[rune]{Bounds: bounds, Values: values}

	return WordSearch{grid, xCoords, aCoords}
}

func part1(input string) int {
	var directions = grid.DiagonalOffsets()

	ws := parseInput(input)

	result := 0
	for _, x := range ws.xCoords {
		for _, offset := range directions {
			m := grid.AddCoords(&x, &offset)
			if c, f := grid.At(&ws.grid, &m); f && *c == 'M' {
				a := grid.AddCoords(&m, &offset)
				if c, f := grid.At(&ws.grid, &a); f && *c == 'A' {
					s := grid.AddCoords(&a, &offset)
					if c, f := grid.At(&ws.grid, &s); f && *c == 'S' {
						result++
					}
				}
			}
		}
	}
	return result
}

func part2(input string) int {
	ulo := grid.MakeCoord(-1, -1)
	uro := grid.MakeCoord(1, -1)
	dlo := grid.MakeCoord(-1, 1)
	dro := grid.MakeCoord(1, 1)

	ws := parseInput(input)

	result := 0
	for _, a := range ws.aCoords {
		ulc := grid.AddCoords(&a, &ulo)
		urc := grid.AddCoords(&a, &uro)
		dlc := grid.AddCoords(&a, &dlo)
		drc := grid.AddCoords(&a, &dro)

		ul, _ := grid.At(&ws.grid, &ulc)
		ur, _ := grid.At(&ws.grid, &urc)
		dl, _ := grid.At(&ws.grid, &dlc)
		dr, _ := grid.At(&ws.grid, &drc)

		if ((*ul == 'M' && *dr == 'S') || (*ul == 'S' && *dr == 'M')) && ((*ur == 'M' && *dl == 'S') || (*ur == 'S' && *dl == 'M')) {
			result++
		}
	}
	return result
}
