package astar

import (
	"adventofcode/pqueue"
	"adventofcode/stack"
	"adventofcode/util"
)

type Neighbour[T comparable] struct {
	Val  T
	Cost int
}

func Solve[T comparable](start T, atGoal func(T) bool, heuristic func(T) int, findNeighbours func(T) []Neighbour[T]) (int, [][]T, bool) {
	openSet := pqueue.MakePriorityQueue[T]()
	cameFrom := make(map[T][]T)
	gScore := make(map[T]int)

	// add start
	gScore[start] = 0
	openSet.Push(start, heuristic(start))

	// solutions
	winningScore := 0
	winners := []T{}

	for openSet.Len() > 0 {
		curr := openSet.Pop()
		currG := gScore[curr]

		if atGoal(curr) {
			// success!

			// if this isn't the first solution, check if the solutions are getting worse
			if len(winners) > 0 {
				if currG > winningScore {
					// this solution is worse -- halt
					break
				}
			} else {
				// first winner, set the score
				winningScore = currG
			}

			// add to winner
			winners = append(winners, curr)
		} else {
			neighbours := findNeighbours(curr)

			for _, neighbour := range neighbours {
				n, nCost := neighbour.Val, neighbour.Cost
				tentativeG := currG + nCost
				g, ok := gScore[n]
				if !ok || tentativeG < g {
					// either first path to neighbour, or better than a previous path
					cameFrom[n] = []T{curr}
					gScore[n] = tentativeG
					fScore := tentativeG + heuristic(n)
					openSet.Push(n, fScore)
				} else if tentativeG == g {
					// an equivalent approach to the current path to neighbour
					cameFrom[n] = append(cameFrom[n], curr)
				}
			}
		}
	}

	if len(winners) == 0 {
		return -1, [][]T{}, false
	}

	// reconstruct paths to winners and add to the solution
	paths := [][]T{}
	for _, s := range winners {
		paths = append(paths, reconstructPath(cameFrom, s)...)
	}
	return winningScore, paths, true
}

func reconstructPath[T comparable](cameFrom map[T][]T, last T) [][]T {
	openSet := stack.NewStack[[]T](1)
	openSet.Push([]T{last})

	solutions := [][]T{}

	for openSet.Len() > 0 {
		path := openSet.Pop()
		curr := path[len(path)-1]
		prevs, ok := cameFrom[curr]
		if ok {
			for i, prev := range prevs {
				if i < len(prevs)-1 {
					// need to make a copy
					tmp := make([]T, len(path))
					copy(tmp, path)
					openSet.Push(append(tmp, prev))
				} else {
					// we can reuse path
					openSet.Push(append(path, prev))
				}
			}
		} else {
			solutions = append(solutions, util.ReverseSlice(path))
		}
	}

	return solutions
}
