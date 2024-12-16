package stack

import (
	"iter"
)

type Stack[T any] struct {
	data []T
	size int
}

func NewStack[T any](initialCapacity int) Stack[T] {
	return Stack[T]{
		data: make([]T, initialCapacity),
		size: 0,
	}
}

func (st *Stack[T]) Len() int {
	return st.size
}

func (st *Stack[T]) Push(val T) *T {
	if st.size == len(st.data) {
		st.grow()
	}
	st.data[st.size] = val
	st.size++
	return &st.data[st.size-1]
}

func (st *Stack[T]) Pop() T {
	if st.size == 0 {
		panic("Can't pop from empty queue")
	}
	st.size--
	return st.data[st.size]
}

func (st *Stack[T]) Clear() {
	st.size = 0
	clear(st.data)
}

func (st *Stack[T]) grow() {
	newData := make([]T, len(st.data)*2)
	copy(newData, st.data)
	st.data = newData
}

func (st *Stack[T]) Iter() iter.Seq[*T] {
	return func(yield func(*T) bool) {
		for i := st.size - 1; i >= 0; i-- {
			if !yield(&st.data[i]) {
				return
			}
		}
	}
}
