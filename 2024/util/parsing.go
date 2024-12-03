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
