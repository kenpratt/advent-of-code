package util

type Tuple[T, U any] struct {
	First  T
	Second U
}

func MakeTuple[T, U any](f T, s U) Tuple[T, U] {
	return Tuple[T, U]{f, s}
}

func (t Tuple[T, U]) Values() (T, U) {
	return t.First, t.Second
}
