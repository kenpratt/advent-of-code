package astar

import (
	"adventofcode/pqueue"
	"adventofcode/stack"
	"adventofcode/util"
	"fmt"
)

type ClientInterface[T comparable] interface {
	AtGoal(val T) bool
	Heuristic(val T) int
	Neighbours(val T) []Neighbour[T]
}

type Neighbour[T comparable] struct {
	Val  T
	Cost int
}

type Metadata[T any] struct {
	gScore   int
	cameFrom []T
	ref      *pqueue.Item[T]
}

func (m *Metadata[T]) setCameFrom(val T, findPath FindPath) {
	if findPath != None {
		if m.cameFrom == nil {
			m.cameFrom = []T{val}
		} else if len(m.cameFrom) == 1 {
			m.cameFrom[0] = val
		} else {
			m.cameFrom = m.cameFrom[:1]
			m.cameFrom[0] = val
		}
	}
}

func (m *Metadata[T]) appendCameFrom(val T, findPath FindPath) {
	if findPath == All {
		m.cameFrom = append(m.cameFrom, val)
	}
}

type FindPath uint8

const (
	None FindPath = iota
	One
	All
)

const printStats = false

func Solve[T comparable](start T, impl ClientInterface[T], findPath FindPath, extra ...int) (int, [][]T, bool) {
	// two extra params to tune openSet and metadata initial capacity
	openSetCapacity := 100
	metadataCapacity := 1000
	if len(extra) > 0 {
		openSetCapacity = extra[0]
		metadataCapacity = extra[1]
	}

	openSet := pqueue.MakePriorityQueue[T](openSetCapacity)
	metadata := make(map[T]*Metadata[T], metadataCapacity)

	// add start
	startRef := openSet.Push(start, impl.Heuristic(start))
	metadata[start] = &Metadata[T]{gScore: 0, ref: startRef}
	maxMetadataSize := len(metadata) // for tuning openSet size

	// solutions
	winningScore := -1
	winners := []T{}

	iters := 0
	for openSet.Len() > 0 {
		iters++
		curr, currF := openSet.Pop()
		currG := metadata[curr].gScore

		if winningScore != -1 && currF > winningScore {
			// can't have any more winners, since F is <= G by definition
			break
		}

		if impl.AtGoal(curr) {
			// success!

			// if this isn't the first solution, check if the solutions are getting worse
			if winningScore != -1 {
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
			neighbours := impl.Neighbours(curr)

			for _, neighbour := range neighbours {
				n, nCost := neighbour.Val, neighbour.Cost
				tentativeG := currG + nCost
				nm, ok := metadata[n]

				if !ok {
					// first path to this node
					fScore := tentativeG + impl.Heuristic(n)
					ref := openSet.Push(n, fScore)
					nm := Metadata[T]{gScore: tentativeG, ref: ref}
					nm.setCameFrom(curr, findPath)
					metadata[n] = &nm
					maxMetadataSize = max(maxMetadataSize, len(metadata))
				} else if tentativeG < nm.gScore {
					// found something better than the previous path

					// update metadata with new score/path
					nm.setCameFrom(curr, findPath)
					nm.gScore = tentativeG

					// reprioritize n in openSet
					fScore := tentativeG + impl.Heuristic(n)
					openSet.Reprioritize(nm.ref, fScore)
				} else if tentativeG == nm.gScore {
					// equivalent approach to the previous path to neighbour
					nm.appendCameFrom(curr, findPath)
				}
			}
		}
	}

	if printStats {
		fmt.Printf("astar stats:\n  iters: %d\n  metadata: %d\n  remaining open set: %d\n  open set max size: %d\n  metadata max size: %d\n", iters, len(metadata), openSet.Len(), openSet.MaxSize, maxMetadataSize)
	}

	if len(winners) == 0 {
		return -1, [][]T{}, false
	}

	// reconstruct paths to winners and add to the solution
	paths := [][]T{}
	if findPath != None {
		for _, s := range winners {
			paths = append(paths, reconstructPath(metadata, s)...)
		}
	}
	return winningScore, paths, true
}

func reconstructPath[T comparable](metadata map[T]*Metadata[T], last T) [][]T {
	openSet := stack.NewStack[[]T](1)
	openSet.Push([]T{last})

	solutions := [][]T{}

	for openSet.Len() > 0 {
		path := openSet.Pop()
		curr := path[len(path)-1]
		m, ok := metadata[curr]
		if ok && len(m.cameFrom) > 0 {
			prevs := m.cameFrom
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
