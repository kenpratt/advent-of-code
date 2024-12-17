package util

func ReverseSlice[T any](a []T) []T {
	l := len(a)
	res := make([]T, l)
	for i, v := range a {
		j := l - i - 1
		res[j] = v
	}
	return res
}
