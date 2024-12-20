Run all days:
$ go run .

Run all tests:
$ go test ./...

Run a specific test
$ cd day05
$ go test -run TestPart2Input

Run gocritic to check for lint/etc
$ gocritic check -enable='#opinionated' ./...

Benchmark CPU:
$ cd day05
$ go test -bench='BenchmarkPart2' -cpuprofile='cpu.prof' -memprofile='mem.prof' -benchtime='20x' && go tool pprof -lines -png cpu.prof > cpu.png

Benchmark CPU and memory:
$ cd day05
$ go test -bench='Benchmark' -cpuprofile='cpu.prof' -memprofile='mem.prof' -benchtime='20x' -benchmem && go tool pprof -lines -png cpu.prof > cpu.png && go tool pprof -lines -png mem.prof > mem.png