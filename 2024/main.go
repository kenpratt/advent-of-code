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
	"adventofcode/day14"
	"adventofcode/day15"
	"adventofcode/day16"
	"adventofcode/day17"
	"adventofcode/util"
	"cmp"
	"flag"
	"fmt"
	"os"
	"slices"
	"strings"
	"testing"

	"github.com/samber/lo"
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
		{"day14", day14.Solve},
		{"day15", day15.Solve},
		{"day16", day16.Solve},
		{"day17", day17.Solve},
	}
}

func daySpecsMap() map[string]Day {
	return lo.Associate(daySpecs(), func(day Day) (string, Day) { return day.name, day })
}

type Result struct {
	name string
	res  testing.BenchmarkResult
}

func main() {
	// set up benchmarking params
	testing.Init()
	flag.Set("test.benchtime", "10x")

	args := os.Args[1:]
	if len(args) > 0 {
		days := strings.Split(args[0], ",")
		times := 1

		if len(args) >= 2 {
			times = util.StringToInt(args[1])
		}
		results := runSome(days, times)
		printSummary(results, false)
	} else {
		results := runAll()
		printSummary(results, true)
	}
}

func runSome(daysToRun []string, times int) []Result {
	days := daySpecsMap()
	results := make([]Result, len(daysToRun)*times)
	for i, dayToRun := range daysToRun {
		day, ok := days[dayToRun]
		util.AssertEqual(true, ok)

		for t := 0; t < times; t++ {
			j := i*times + t
			results[j] = runDay(day)
		}
		slices.SortFunc(results[i*times:(i+1)*times], func(a, b Result) int { return cmp.Compare(a.res.T, b.res.T) })
	}
	return results
}

func runAll() []Result {
	// benchmark all the days
	days := daySpecs()
	results := make([]Result, len(days))
	for i, day := range days {
		results[i] = runDay(day)
	}
	return results
}

func printSummary(results []Result, topN bool) {
	// print summary
	fmt.Printf("\nSummary:\n")
	totalNs, totalBytes, totalAllocs := 0, 0, 0
	for _, day := range results {
		fmt.Printf("%s: %s\t%s\n", day.name, day.res.String(), day.res.MemString())
		totalNs += int(day.res.NsPerOp())
		totalBytes += int(day.res.AllocedBytesPerOp())
		totalAllocs += int(day.res.AllocsPerOp())
	}

	fmt.Printf("\nTotal:\n  Runtime:     %6d ms\n  Memory:      %6d kB\n  Allocations: %6d allocs\n", totalNs/1000000, totalBytes/1000, totalAllocs)

	if topN {
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
