package astar

import (
	"adventofcode/pqueue"
	"adventofcode/util"
)

func Solve[T comparable](start T, atGoal func(T, int) bool, heuristic func(T) int, findNeighbours func(T) []util.Tuple[T, int]) (int, bool) {
	openSet := pqueue.MakePriorityQueue[T]()

	// TODO store gScore on the node? eg make new struct for PQueue of Tuple[T, int] where second is gScore
	// then I don't need a map
	gScore := make(map[T]int)

	// add start
	gScore[start] = 0
	openSet.Push(start, heuristic(start))

	for openSet.Len() > 0 {
		curr := openSet.Pop()
		currG := gScore[curr]

		if atGoal(curr, currG) { // TODO remove second arg
			// success!
			return currG, true
		}

		neighbours := findNeighbours(curr)

		// TODO openSet not always in correct order? is that expected?

		for _, tuple := range neighbours {
			n, nCost := tuple.Values() // TODO any better mem/CPU to do tuple.First, tuple.Second?
			tentativeG := currG + nCost
			if g, ok := gScore[n]; !ok || tentativeG < g {
				// either first path to neighbour, or better than a previous path
				gScore[n] = tentativeG
				fScore := tentativeG + heuristic(n) // TODO cache heuristic per N?
				openSet.Push(n, fScore)             // TODO check if elem already exists and rescore?
			}
		}
	}

	// no solution
	return -1, false
}
