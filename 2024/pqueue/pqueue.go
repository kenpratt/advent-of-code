package pqueue

import (
	"container/heap"
	"fmt"
)

type PriorityQueue[T any] struct {
	data PriorityQueueImpl[T]
}

func MakePriorityQueue[T any]() PriorityQueue[T] {
	data := make(PriorityQueueImpl[T], 0)
	heap.Init(&data)
	return PriorityQueue[T]{data}
}

func (pq *PriorityQueue[T]) Push(val T, priority int) {
	item := Item[T]{value: val, priority: priority}
	heap.Push(&pq.data, &item)
}

func (pq *PriorityQueue[T]) Pop() T {
	item := heap.Pop(&pq.data).(*Item[T])
	return item.value
}

func (pq *PriorityQueue[T]) Len() int {
	return pq.data.Len()
}

// TODO remove
func (pq *PriorityQueue[T]) Print() {
	fmt.Println("PQ:")
	for _, it := range pq.data {
		fmt.Println("  ", *it)
	}
}

// from https://pkg.go.dev/container/heap#example-package-PriorityQueue

// An Item is something we manage in a priority queue.
type Item[T any] struct {
	value    T   // The value of the item; arbitrary.
	priority int // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueueImpl[T any] []*Item[T]

func (pq PriorityQueueImpl[T]) Len() int { return len(pq) }

func (pq PriorityQueueImpl[T]) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueueImpl[T]) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueueImpl[T]) Push(x any) {
	n := len(*pq)
	item := x.(*Item[T])
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueueImpl[T]) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // don't stop the GC from reclaiming the item eventually
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}
