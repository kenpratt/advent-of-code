package util

import "fmt"

func AssertEqual[T comparable](expected T, actual T) {
	if expected != actual {
		panic(fmt.Sprintf("assertion failed (expected %v, was %v)", expected, actual))
	}
}
