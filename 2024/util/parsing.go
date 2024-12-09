package util

import "strconv"

func StringToInt(s string) int {
	v, err := strconv.Atoi(s)
	if err != nil {
		// ... handle error
		panic(err)
	}
	return v
}

func RuneToInt(c rune) int {
	return int(c - '0')
}
