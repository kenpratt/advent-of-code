package main

import (
	"adventofcode/day01"
	"adventofcode/day02"
	"adventofcode/day03"
	"adventofcode/day04"
	"adventofcode/day05"
	"adventofcode/day06"
	"adventofcode/day07"
	"adventofcode/day08"
	"adventofcode/day09"
	"adventofcode/day10"
	"adventofcode/day11"
	"adventofcode/day12"
	"adventofcode/day13"
	"cmp"
	"flag"
	"fmt"
	"os"
	"slices"
	"testing"
)

type Day struct {
	name  string
	solve func(string)
}

func daySpecs() []Day {
	return []Day{
		{"day01", day01.Solve},
		{"day02", day02.Solve},
		{"day03", day03.Solve},
		{"day04", day04.Solve},
		{"day05", day05.Solve},
		{"day06", day06.Solve},
		{"day07", day07.Solve},
		{"day08", day08.Solve},
		{"day09", day09.Solve},
		{"day10", day10.Solve},
		{"day11", day11.Solve},
		{"day12", day12.Solve},
		{"day13", day13.Solve},
	}
}

type Result struct {
	name string
	res  testing.BenchmarkResult
}

func main() {
	args := os.Args[1:]
	fmt.Println("running with args", args)
	// TODO actually do something with args, or remove it?

	// set up benchmarking params
	testing.Init()
	flag.Set("test.benchtime", "10x")

	// benchmark all the days
	days := daySpecs()
	results := make([]Result, len(days))
	for i, day := range days {
		results[i] = runDay(day)
	}

	// print summary
	fmt.Printf("\nSummary:\n")
	totalNs, totalBytes, totalAllocs := 0, 0, 0
	for _, day := range results {
		fmt.Printf("%s: %s\t%s\n", day.name, day.res.String(), day.res.MemString())
		totalNs += int(day.res.NsPerOp())
		totalBytes += int(day.res.AllocedBytesPerOp())
		totalAllocs += int(day.res.AllocsPerOp())
	}

	fmt.Printf("\nTotal (per op):\n  Runtime:     %6d ms\n  Memory:      %6d kB\n  Allocations: %6d allocs\n", totalNs/1000000, totalBytes/1000, totalAllocs)

	// top 5 by time taken
	fmt.Printf("\nSlowest:\n")
	slices.SortFunc(results, func(a, b Result) int {
		return -cmp.Compare(a.res.NsPerOp(), b.res.NsPerOp())
	})
	for i := 0; i < 5; i++ {
		day := results[i]
		fmt.Printf("  %s: %6d ms\n", day.name, day.res.NsPerOp()/1000000)
	}

	// top 5 by memory used
	fmt.Printf("\nHighest memory:\n")
	slices.SortFunc(results, func(a, b Result) int {
		return -cmp.Compare(a.res.AllocedBytesPerOp(), b.res.AllocedBytesPerOp())
	})
	for i := 0; i < 5; i++ {
		day := results[i]
		fmt.Printf("  %s: %6d kB\n", day.name, day.res.AllocedBytesPerOp()/1000)
	}

	// top 5 by memory allocations
	fmt.Printf("\nHeaviest allocations:\n")
	slices.SortFunc(results, func(a, b Result) int {
		return -cmp.Compare(a.res.AllocsPerOp(), b.res.AllocsPerOp())
	})
	for i := 0; i < 5; i++ {
		day := results[i]
		fmt.Printf("  %s: %6d allocs\n", day.name, day.res.AllocsPerOp())
	}
}

func runDay(day Day) Result {
	fmt.Println("Running", day.name)

	res := testing.Benchmark(func(b *testing.B) {
		for n := 0; n < b.N; n++ {
			day.solve(day.name)
		}
	})

	return Result{day.name, res}
}
