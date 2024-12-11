package stack

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

func (st *Stack[T]) Push(val T) {
	if st.size == len(st.data) {
		st.grow()
	}
	st.data[st.size] = val
	st.size++
}

func (st *Stack[T]) Pop() T {
	if st.size == 0 {
		panic("Can't pop from empty queue")
	}
	st.size--
	val := st.data[st.size]
	return val
}

func (st *Stack[T]) Clear() {
	st.size = 0
}

func (st *Stack[T]) grow() {
	newData := make([]T, len(st.data)*2)
	copy(newData, st.data)
	st.data = newData
}
