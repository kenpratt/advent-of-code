package util

import (
	"os"
	"path/filepath"
)

func ReadInputFile(path string) string {
	input, err := os.ReadFile(filepath.Join(path, "input.txt"))
	if err != nil {
		panic(err)
	}

	return string(input)
}
