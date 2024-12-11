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

func NumDigits(val int) int {
	digits := 0
	for val > 0 {
		val /= 10
		digits++
	}
	return digits
}

func ConcatenateInts(l, r int) int {
	t := r
	for t > 0 {
		l *= 10
		t /= 10
	}
	return l + r
}

func SplitInts(val, digits int) (int, int) {
	left := val
	right := 0

	place := 1
	for digits > 0 {
		v := left % 10
		left /= 10
		right += v * place

		digits--
		place *= 10
	}

	return left, right
}
