package day17

import (
	"adventofcode/stack"
	"adventofcode/util"
	"fmt"

	"github.com/samber/lo"
)

type VariableComputer struct {
	program []uint8
	queue   stack.Stack[State]
}

type State struct {
	a, b, c Value
	ip      int
	op      int
	status  Status
	memory  Memory
}

type Status uint8

const (
	Ready Status = iota + 1
	Running
	Completed
	Failure
)

type Memory [64]BitValue

func MakeMemory() Memory {
	m := Memory{}
	for i := range m {
		m[i] = Unknown
	}
	return m
}

func (m *Memory) LowestPossibleValue() int {
	res := 0
	for _, b := range m {
		res <<= 1 // shift left

		switch b {
		case Zero, Unknown:
			// treat unknown as zero
			// noop
		case One:
			res += 1
		}
	}
	return res
}

func MakeVariableComputer(input Input, variableRegister string) VariableComputer {
	c := VariableComputer{
		program: input.program,
		queue:   stack.NewStack[State](8),
	}
	s := MakeInitialBranch(input)
	if variableRegister == "a" {
		s.a = s.InitialVariableValue()
	} else {
		panic(fmt.Sprintf("Unknown variableRegister: %v", variableRegister))
	}
	c.AddBranch(s)
	return c
}

func MakeInitialBranch(input Input) State {
	return State{
		a:      StaticValue(input.a),
		b:      StaticValue(input.b),
		c:      StaticValue(input.c),
		ip:     0,
		op:     0,
		status: Ready,
		memory: MakeMemory(),
	}
}

func (s *State) InitialVariableValue() Value {
	bits := make([]Bit, len(s.memory))
	for i := 0; i < len(s.memory); i++ {
		bits[i] = Bit{
			index:   i,
			flipped: false,
		}
	}
	return VariableValue(bits, s)
}

func (c *VariableComputer) AddBranch(s State) {
	c.queue.Push(s)
}

func (b *State) Clone() State {
	c := State{
		a:      b.a.Clone(),
		b:      b.b.Clone(),
		c:      b.c.Clone(),
		ip:     b.ip,
		op:     b.op,
		status: Ready,
	}
	copy(c.memory[:], b.memory[:])
	return c
}

func (c *VariableComputer) Solve() int {
	solutions := make([]int, 0)

	for c.queue.Len() > 0 {
		s := c.queue.Pop()

		util.AssertEqual(Ready, s.status)
		s.status = Running
		for s.status == Running {
			c.ExecuteNextInstruction(&s)
		}

		if s.status == Completed {
			v := s.memory.LowestPossibleValue()
			solutions = append(solutions, v)
		}
	}

	return lo.Min(solutions)
}

func (comp *VariableComputer) ExecuteNextInstruction(s *State) {
	if s.ip >= len(comp.program) {
		s.status = Failure
		return
	}

	inst, val := comp.program[s.ip], comp.program[s.ip+1]
	switch inst {
	case 0, 6, 7:
		// adv/bdv/cdv: A / 2^combo -> A/B/C
		c := s.combo(val)

		// if combo has unknowns, we need to branch into N states,
		// one per possible value
		if c.variable {
			// all remaining unknowns in the value
			unknowns := c.Unknowns(s)

			// should have unknowns if it's still set to variable
			util.AssertEqual(true, len(unknowns) > 0)

			// all possible permutations
			permutations := boolPermutations(len(unknowns))

			// add branches
			for _, vals := range permutations[1:] {
				b := s.Clone()
				b.SetBits(unknowns, vals)
				comp.AddBranch(b)
			}

			// set ths branch to the first permutation
			s.SetBits(unknowns, permutations[0])
			c = VariableValue(c.bits, s) // should return a static value
			util.AssertEqual(false, c.variable)
		}

		// A / 2^combo == A shifted right by combo
		res := s.Shr(s.a, c)

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
		res := s.Xor(s.b, StaticValue(int(val)))
		s.b = res
		s.ip += 2
	case 2:
		// bst: combo % 8 -> B
		c := s.combo(val)
		res := s.Mod8(c)
		s.b = res
		s.ip += 2
	case 3:
		// jnz: if A != 0, jump to val

		var jump bool

		if s.a.variable {
			// does the variable contain any fixed 1 values? if so it can't be zero
			if _, containsAOne := lo.Find(s.a.bits, func(b Bit) bool { return s.BitValue(b) == One }); containsAOne {
				// can't be zero, jump
				jump = true
			} else {
				// it could be zero or not, we'll need to branch
				// - create a branch with all unknowns set to zero
				// - this branch can carry on as-is assuming a non-zero value

				// all remaining unknowns in the value
				unknowns := s.a.Unknowns(s)

				// should have unknowns if it's still set to variable
				util.AssertEqual(true, len(unknowns) > 0)

				// add a branch with all zero
				b := s.Clone()
				b.SetBits(unknowns, make([]bool, len(unknowns)))
				comp.AddBranch(b)

				// assume this branch is non-zero
				jump = true
			}
		} else {
			// static value, jump if it's not zero
			jump = s.a.static != 0
		}

		if jump {
			s.ip = int(val)
		} else {
			s.ip += 2
		}
	case 4:
		// bxc: B XOR C -> B
		res := s.Xor(s.b, s.c)
		s.b = res
		s.ip += 2
	case 5:
		// out: combo % 8 -> output
		c := s.combo(val)
		res := s.Mod8(c)

		// instead of appending output, this informs us about certain values of unknowns
		output := comp.program[s.op]

		if !res.variable {
			// we have a static, easy to check if it's correct or not
			if res.static != int(output) {
				s.status = Failure
				return
			}
			// correct output, we can continue
		} else {
			wantVals := bits(output)
			util.AssertEqual(len(res.bits), len(wantVals))

			bitsToSet := make([]Bit, 0)
			valsToSet := make([]bool, 0)

			for i, want := range wantVals {
				bit := res.bits[i]
				have := s.BitValue(bit)

				if have == Unknown {
					// set this bit
					bitsToSet = append(bitsToSet, bit)
					valsToSet = append(valsToSet, want)
				} else if (have == Zero && want) || (have == One && !want) {
					s.status = Failure
					return
				} else {
					// otherwise, we can ignore this bit as it's already the correct value
				}
			}

			if len(bitsToSet) > 0 {
				s.SetBits(bitsToSet, valsToSet)
			}
		}

		s.op++
		s.ip += 2

		if s.op == len(comp.program) {
			s.status = Completed
		}
	default:
		panic("Unreachable")
	}
}

func (s *State) combo(val uint8) Value {
	switch val {
	case 0:
		return StaticValue(0)
	case 1:
		return StaticValue(1)
	case 2:
		return StaticValue(2)
	case 3:
		return StaticValue(3)
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

func (s *State) Shr(v, n Value) Value {
	if n.variable {
		panic("Unsupported: Shr by variable amount")
	}

	if v.variable {
		// truncate n bits
		toCopy := len(v.bits) - n.static
		if toCopy > 0 {
			bits := make([]Bit, toCopy)
			copy(bits, v.bits[0:toCopy])
			res := VariableValue(bits, s)
			return res
		} else {
			// truncating all the bits!
			return StaticValue(0)
		}
	} else {
		return StaticValue(v.static >> n.static)
	}
}

func (s *State) setBit(b Bit, v bool) {
	if s.memory[b.index] != Unknown {
		panic(fmt.Sprintf("Trying to set a bit that's already set: setting %v to %v (existing value: %v)", b, v, s.memory[b.index]))
	}

	// apply flip
	if b.flipped {
		v = !v
	}

	// set memory
	s.memory[b.index] = boolToBitValue(v)
}

func (s *State) FlipBit(b *Bit) {
	b.flipped = !b.flipped
}

func boolPermutations(n int) [][]bool {
	vals := make([][]bool, util.IntPow(2, n))
	for i := range vals {
		vals[i] = make([]bool, n)
	}

	// eg:
	// 0 0 0
	// 0 0 1
	// 0 1 0
	// 0 1 1
	// 1 0 0
	// 1 0 1
	// 1 1 0
	// 1 1 1
	div := util.IntPow(2, n-1)
	for u := range n {
		for i := range vals {
			vals[i][u] = i/div%2 == 1
		}
		div /= 2
	}
	return vals
}

func (s *State) Mod8(v Value) Value {
	if v.variable {
		// mod 8 doesn't tell us anything about any of the bits besides the rightmost 3 bits
		bits := make([]Bit, 3)

		if len(v.bits) >= 3 {
			startAt := len(v.bits) - 3
			copy(bits, v.bits[startAt:])
		} else {
			// need some extra zeroes
			zeroes := 3 - len(v.bits)
			copy(bits[zeroes:], v.bits)
			for i := 0; i < zeroes; i++ {
				bits[i] = Bit{index: -1, flipped: false, fixed: false}
			}
		}

		return VariableValue(bits, s)
	} else {
		return StaticValue(v.static % 8)
	}
}

func (s *State) Xor(v Value, o Value) Value {
	if !v.variable && o.variable {
		// re-do with order flipped
		return s.Xor(o, v)
	}

	if o.variable {
		panic("Unsupported: XOR with variable right operand")
	}

	if v.variable {
		if o.static > 7 {
			panic("Unsupported: XOR with static value >7")
		}

		// start with a copy
		var resBits []Bit
		if len(v.bits) >= 3 {
			// copy directly
			resBits = make([]Bit, len(v.bits))
			copy(resBits, v.bits)
		} else {
			// not enough bits, need to add some bits fixed at 0
			resBits = make([]Bit, 3)
			zeroes := 3 - len(v.bits)
			copy(resBits[zeroes:], v.bits)
			for i := 0; i < zeroes; i++ {
				resBits[i] = Bit{index: -1, flipped: false, fixed: false}
			}
		}

		// now flip any bits with a value of 1
		start := len(resBits) - 3
		vals := bits(uint8(o.static))
		for i, b := range vals {
			if b {
				s.FlipBit(&resBits[start+i])
			}
		}

		return VariableValue(resBits, s)
	} else {
		return StaticValue(v.static ^ o.static)
	}
}

func (s *State) MemoryValue(i int) BitValue {
	return s.memory[i]
}

func (s *State) BitValue(b Bit) BitValue {
	var v BitValue

	if b.index == -1 {
		// special case, fixed value
		v = boolToBitValue(b.fixed)
	} else {
		v = s.MemoryValue(b.index)
	}

	if b.flipped {
		return v.Flipped()
	} else {
		return v
	}
}

func (s *State) SetBits(bits []Bit, vals []bool) {
	util.AssertEqual(len(bits), len(vals))
	for i, b := range bits {
		v := vals[i]
		s.setBit(b, v)
	}

	// refresh the register values, in case they are static now
	s.a = VariableValue(s.a.bits, s)
	s.b = VariableValue(s.b.bits, s)
	s.c = VariableValue(s.c.bits, s)
}

func bits(v uint8) [3]bool {
	return [3]bool{v&4 != 0, v&2 != 0, v&1 != 0}
}

type Bit struct {
	index   int
	flipped bool
	fixed   bool
}

type BitValue uint8

const (
	Zero BitValue = iota
	One
	Unknown
)

func boolToBitValue(b bool) BitValue {
	if b {
		return One
	} else {
		return Zero
	}
}

func (b *BitValue) Flipped() BitValue {
	switch *b {
	case Zero:
		return One
	case One:
		return Zero
	case Unknown:
		return Unknown
	default:
		panic("Unreachable")
	}
}

type Value struct {
	variable bool
	static   int   // used for static value
	bits     []Bit // used for variable value
}

func (v Value) Clone() Value {
	r := v
	r.bits = make([]Bit, len(v.bits))
	copy(r.bits, v.bits)
	return r
}

func StaticValue(val int) Value {
	return Value{variable: false, static: val}
}

func VariableValue(bits []Bit, s *State) Value {
	// Check if all the bits in here are known, and if so, convert to a static value
	val := 0
	for _, b := range bits {
		val <<= 1
		switch s.BitValue(b) {
		case Unknown:
			// at least one Unknown
			return Value{variable: true, bits: bits}
		case Zero:
			// noop
		case One:
			val += 1
		}
	}
	return StaticValue(val)
}

func (v *Value) Unknowns(s *State) []Bit {
	if !v.variable {
		panic("Finding unknowns supported for variables only")
	}

	return lo.Filter(v.bits, func(b Bit, _ int) bool { return s.BitValue(b) == Unknown })
}
