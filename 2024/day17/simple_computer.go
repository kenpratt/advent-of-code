package day17

import (
	"strconv"
	"strings"

	"github.com/samber/lo"
)

type SimpleComputer struct {
	a, b, c int
	program []uint8
	ip      int
	output  []int
}

func MakeSimpleComputer(input Input) SimpleComputer {
	return SimpleComputer{input.a, input.b, input.c, input.program, 0, []int{}}
}

func (s *SimpleComputer) Run() {
	for s.ip < len(s.program) {
		s.execute(s.program[s.ip], s.program[s.ip+1])
	}
}

func (s *SimpleComputer) OutputString() string {
	return strings.Join(lo.Map(s.output, func(x int, _ int) string { return strconv.Itoa(x) }), ",")
}

func (s *SimpleComputer) execute(inst, val uint8) {
	switch inst {
	case 0, 6, 7:
		// adv/bdv/cdv: A / 2^combo -> A/B/C
		c := s.combo(val)

		// A / 2^combo == A shifted right by combo
		res := s.a >> c

		switch inst {
		case 0:
			s.a = res
		case 6:
			s.b = res
		case 7:
			s.c = res
		}

		s.ip += 2
	case 1:
		// bxl: B XOR val -> B
		res := s.b ^ int(val)
		s.b = res
		s.ip += 2
	case 2:
		// bst: combo % 8 -> B
		c := s.combo(val)
		res := c % 8
		s.b = res
		s.ip += 2
	case 3:
		// jnz: if A != 0, jump to val
		if s.a != 0 {
			s.ip = int(val)
		} else {
			s.ip += 2
		}
	case 4:
		// bxc: B XOR C -> B
		res := s.b ^ s.c
		s.b = res
		s.ip += 2
	case 5:
		// out: combo % 8 -> output
		c := s.combo(val)
		res := c % 8
		s.output = append(s.output, res)
		s.ip += 2
	default:
		panic("Unreachable")
	}
}

func (s *SimpleComputer) combo(val uint8) int {
	switch val {
	case 0:
		return 0
	case 1:
		return 1
	case 2:
		return 2
	case 3:
		return 3
	case 4:
		return s.a
	case 5:
		return s.b
	case 6:
		return s.c
	case 7:
		panic("Combo operand 7 is reserved and will not appear in valid programs")
	default:
		panic("Unreachable")
	}
}
