package pqueue

import (
	"container/heap"
)

type PriorityQueue[T any] struct {
	data    PriorityQueueImpl[T]
	MaxSize int // for tuning the capacity
}

func MakePriorityQueue[T any](extra ...int) PriorityQueue[T] {
	capacity := 100
	if len(extra) > 0 {
		capacity = extra[0]
	}
	data := make(PriorityQueueImpl[T], 0, capacity)
	heap.Init(&data)
	return PriorityQueue[T]{data, 0}
}

func (pq *PriorityQueue[T]) Push(val T, priority int) *Item[T] {
	item := Item[T]{value: val, priority: priority}
	heap.Push(&pq.data, &item)
	pq.MaxSize = max(pq.MaxSize, pq.Len())
	return &item
}

func (pq *PriorityQueue[T]) Pop() (T, int) {
	item := heap.Pop(&pq.data).(*Item[T])
	return item.value, item.priority
}

func (pq *PriorityQueue[T]) Len() int {
	return pq.data.Len()
}

func (pq *PriorityQueue[T]) Reprioritize(item *Item[T], priority int) {
	item.priority = priority
	heap.Fix(&pq.data, item.index)
}

func (pq *PriorityQueue[T]) Clear() {
	clear(pq.data)
	pq.MaxSize = 0
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
