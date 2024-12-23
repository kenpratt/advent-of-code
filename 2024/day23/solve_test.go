package day23

import (
	"adventofcode/util"
	"testing"

	"github.com/stretchr/testify/assert"
)

const example = `kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn`

func TestPart1Example(t *testing.T) {
	expected := 7
	input := parseInput(example)
	actual := part1(&input)
	assert.Equal(t, expected, actual)
}

func TestPart1Input(t *testing.T) {
	expected := 1330
	input := parseInput(util.ReadInputFile("."))
	actual := part1(&input)
	assert.Equal(t, expected, actual)
}

func TestPart2Example(t *testing.T) {
	expected := "co,de,ka,ta"
	input := parseInput(example)
	actual := part2(&input)
	assert.Equal(t, expected, actual)
}

func TestPart2Input(t *testing.T) {
	expected := "hl,io,ku,pk,ps,qq,sh,tx,ty,wq,xi,xj,yp"
	input := parseInput(util.ReadInputFile("."))
	actual := part2(&input)
	assert.Equal(t, expected, actual)
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		input := parseInput(util.ReadInputFile("."))
		part1(&input)
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		input := parseInput(util.ReadInputFile("."))
		part2(&input)
	}
}
