package set

import (
	"iter"
	"maps"
)

type Set[T comparable] struct {
	values map[T]struct{}
}

func NewSet[T comparable](initialVals ...T) Set[T] {
	s := Set[T]{
		values: make(map[T]struct{}, len(initialVals)),
	}
	for _, val := range initialVals {
		s.Add(val)
	}
	return s
}

func (s *Set[T]) Len() int {
	return len(s.values)
}

func (s *Set[T]) Add(val T) {
	s.values[val] = struct{}{}
}

func (s *Set[T]) Contains(val T) bool {
	_, ok := s.values[val]
	return ok
}

func (s *Set[T]) Iter() iter.Seq[T] {
	return maps.Keys(s.values)
}

func (s *Set[T]) Remove(val T) {
	delete(s.values, val)
}
