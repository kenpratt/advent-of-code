Run all days:
$ go run .

Run all tests:
$ go test ./...

Run a specific test
$ cd day05
$ go test -run TestPart2Input

Run gocritic to check for lint/etc
$ gocritic check -enable='#opinionated' ./...

Benchmark a certain day:
$ cd day05
$ go test -bench='BenchmarkPart2' -cpuprofile='cpu.prof' -memprofile='mem.prof' -benchtime='10x' -benchmem