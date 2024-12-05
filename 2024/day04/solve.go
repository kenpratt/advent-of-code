package day04

import (
	"adventofcode/util"
	"fmt"
	"strings"
)

func Solve(path string) {
	input := util.ReadInputFile(path)
	fmt.Println("part 1: ", part1(input))
	fmt.Println("part 2: ", part2(input))
}

type Coord struct {
	x int
	y int
}

type Bounds struct {
	width  int
	height int
}

type Grid struct {
	bounds Bounds
	values []rune
}

type WordSearch struct {
	grid    Grid
	xCoords []Coord
}

func parseInput(input string) WordSearch {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])
	bounds := Bounds{width, height}
	values := make([]rune, width*height)
	xCoords := make([]Coord, 0)

	for y, line := range lines {
		for x, char := range line {
			pos := Coord{x, y}

			if char == 'X' {
				xCoords = append(xCoords, pos)
			}

			values[coordToIndex(bounds, pos)] = char
		}
	}

	grid := Grid{bounds, values}

	return WordSearch{grid, xCoords}
}

func coordToIndex(bounds Bounds, pos Coord) int {
	return pos.y*bounds.width + pos.x
}

func inBounds(bounds Bounds, pos Coord) bool {
	return pos.y >= 0 && pos.y < bounds.height && pos.x >= 0 && pos.x < bounds.width
}

func addCoords(c1 Coord, c2 Coord) Coord {
	x := c1.x + c2.x
	y := c1.y + c2.y
	return Coord{x, y}
}

func gridAt(grid Grid, pos Coord) rune {
	if inBounds(grid.bounds, pos) {
		index := coordToIndex(grid.bounds, pos)
		return grid.values[index]
	} else {
		return -1
	}
}

func part1(input string) int {
	var directions = [...]Coord{{-1, -1}, {0, -1}, {1, -1}, {-1, 0}, {1, 0}, {-1, 1}, {0, 1}, {1, 1}}

	ws := parseInput(input)

	result := 0
	for _, x := range ws.xCoords {
		for _, offset := range directions {
			m := addCoords(x, offset)
			if gridAt(ws.grid, m) == 'M' {
				a := addCoords(m, offset)
				if gridAt(ws.grid, a) == 'A' {
					s := addCoords(a, offset)
					if gridAt(ws.grid, s) == 'S' {
						result++
					}
				}
			}
		}
	}
	return result
}

func part2(input string) int {
	return 0
}
