package util

func AbsInt(x int) int {
	if x >= 0 {
		return x
	} else {
		return -x
	}
}

func AbsDiff(x, y int) int {
	if y >= x {
		return y - x
	} else {
		return x - y
	}
}
