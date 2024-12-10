package day09

import (
	"adventofcode/util"
	"sort"
)

func Solve(path string) {
	inputStr := util.ReadInputFile(path)
	input := parseInput(inputStr)
	util.AssertEqual(6332189866718, part1(input))
	util.AssertEqual(6353648390778, part2(input))
}

func parseInput(input string) [][2]int {
	res := make([][2]int, len(input))
	id := 0

	for i, c := range input {
		size := util.RuneToInt(c)
		if i%2 == 0 {
			// file
			res[i] = [2]int{size, id}
			id++
		} else {
			// gap
			res[i] = [2]int{size, -1}
		}
	}

	return res
}

func part1(diskMap [][2]int) int {
	// count of unmoved blocks for each file (initially, file size)
	fileCount := len(diskMap)/2 + 1
	remainingBlocks := make([]int, fileCount)
	for _, item := range diskMap {
		size := item[0]
		id := item[1]
		if id != -1 {
			remainingBlocks[id] = size
		}
	}

	// calculate checksum as we go
	pos := 0
	checksum := 0
	nextToMove := len(remainingBlocks) - 1
	for _, item := range diskMap {
		size := item[0]
		id := item[1]
		if id != -1 {
			// file: add to checksum, if it hasn't been moved
			remaining := remainingBlocks[id]
			remainingBlocks[id] = 0
			for i := 0; i < remaining; i++ {
				checksum += pos * id
				pos++
			}
		} else {
			// fill gap
			gap := size
			for gap > 0 && nextToMove > -1 {
				// skip over any blocks that are extinquished
				for nextToMove >= 0 && remainingBlocks[nextToMove] == 0 {
					nextToMove--
				}
				if nextToMove < 0 {
					break
				}

				// move one block from nextToMove
				checksum += pos * nextToMove
				pos++
				gap--
				remainingBlocks[nextToMove]--
			}
		}
	}

	return checksum
}

func part2(diskMap [][2]int) int {
	// build metadata about files and gaps
	fileCount := len(diskMap)/2 + 1
	fileSizes := make([]int, fileCount)
	filePositions := make([]int, fileCount)
	gaps := make(map[int][]int)
	maxGap := 0
	pos := 0
	for _, item := range diskMap {
		size := item[0]
		id := item[1]
		if id != -1 {
			// file metadata
			fileSizes[id] = size
			filePositions[id] = pos
		} else {
			// list of gaps of a certain size
			gaps[size] = append(gaps[size], pos)
			maxGap = max(maxGap, size)
		}
		pos += size
	}

	// move files, one at a time
	for id := fileCount - 1; id >= 0; id-- {
		fileSize := fileSizes[id]
		filePos := filePositions[id]

		// find leftmost gap >= size
		gapPos, success := takeLeftmostGap(fileSize, filePos, gaps, maxGap)
		if success {
			filePositions[id] = gapPos
		}
	}

	// calculate checksum with final file positions
	checksum := 0
	for id, pos := range filePositions {
		size := fileSizes[id]
		for i := 0; i < size; i++ {
			checksum += (pos + i) * id
		}
	}
	return checksum
}

func takeLeftmostGap(wantSize int, existingPos int, gaps map[int][]int, maxGap int) (int, bool) {
	// find leftmost gap >= size
	foundOne := false
	leftmostPos := -1
	leftmostSize := -1

	// check gaps >= size
	for size := wantSize; size <= maxGap; size++ {
		if len(gaps[size]) > 0 {
			pos := gaps[size][0]
			if pos < existingPos && (!foundOne || pos < leftmostPos) {
				// found a gap that's more left
				foundOne = true
				leftmostPos = pos
				leftmostSize = size
			}
		}
	}

	if !foundOne {
		return -1, false
	}

	// remove the gap we're returning
	util.AssertEqual(leftmostPos, gaps[leftmostSize][0])
	gaps[leftmostSize] = gaps[leftmostSize][1:]

	// add remainder, if any
	remainder := leftmostSize - wantSize
	if remainder > 0 {
		remainderPos := leftmostPos + wantSize
		gaps[remainder] = append(gaps[remainder], remainderPos)
		sort.Ints(gaps[remainder])
	}

	return leftmostPos, true
}
