require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(raw_input)
  raw_input.strip.split(',').map(&:to_i)
end

def modify_program(input, noun, verb)
  out = input.clone
  out[1] = noun
  out[2] = verb
  out  
end

class Instruction
  attr_reader :opcode

  def initialize(op)
    @op = op
    @opcode = op % 100
  end

  def param_count
    case @opcode
    when 1, 2
      3
    when 3, 4
      1
    when 5, 6
      2
    when 7, 8
      3
    when 99
      0
    else
      raise "Unknown opcode: #{@opcode}"
    end
  end

  def size
    param_count + 1
  end

  def param1_mode
    (@op / 100) % 10
  end

  def param2_mode
    (@op / 1000) % 10
  end

  def param3_mode
    (@op / 10000) % 10
  end
end

class IntcodeComputer
  attr_reader :memory, :output

  def initialize(program, input)
    @memory = program.clone
    @instruction_pointer = 0
    @input = input.clone
    @output = []
    @halt = false
  end

  def self.run(program, input=[])
    computer = IntcodeComputer.new(program, input)
    computer.run
    computer
  end

  def run
    @halt = false
    while !@halt && @instruction_pointer < @memory.size
      op = read(0, 1)
      inst = Instruction.new(op)
      curr_insruction_pointer = @instruction_pointer

      execute(inst)

      # advance instruction pointer, unless the instruction modified it
      if @instruction_pointer == curr_insruction_pointer
        @instruction_pointer += inst.size
      end
    end
    validate_output!
  end

  def execute(inst)
    case inst.opcode
    when 1
      # add (v1 + v2)
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      write(3, inst.param3_mode, val1 + val2)
    when 2
      # multiply (v1 * v2)
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      write(3, inst.param3_mode, val1 * val2)
    when 3
      # takes a single integer as input and saves it to the position given by its only parameter.
      val = @input.shift
      write(1, inst.param1_mode, val)
    when 4
      # outputs the value of its only parameter.
      @output << read(1, inst.param1_mode)
    when 5
      # jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value
      # from the second parameter. Otherwise, it does nothing.
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      if val1 != 0
        @instruction_pointer = val2
      end
    when 6
      # jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value
      # from the second parameter. Otherwise, it does nothing.
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      if val1 == 0
        @instruction_pointer = val2
      end
    when 7
      # less than: if the first parameter is less than the second parameter, it stores 1 in the position
      # given by the third parameter. Otherwise, it stores 0.
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      to_write = val1 < val2 ? 1 : 0
      write(3, inst.param3_mode, to_write)
    when 8
      # Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the
      # position given by the third parameter. Otherwise, it stores 0.
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      to_write = val1 == val2 ? 1 : 0
      write(3, inst.param3_mode, to_write)
    when 99
      @halt = true
    else
      raise "Unknown opcode: #{@opcode}"
    end
  end

  def read(offset, mode)
    case mode
    when 0
      # position mode (indirect)
      pos = @memory[@instruction_pointer + offset]
      @memory[pos]
    when 1
      # immediate mode (direct)
      @memory[@instruction_pointer + offset]
    else
      raise "Unknown read mode: #{mode}"
    end
  end

  def write(offset, mode, to_write)
    case mode
    when 0
      # position mode (indirect)
      pos = @memory[@instruction_pointer + offset]
      @memory[pos] = to_write
    when 1
      raise "Cannot use immedate mode for writes"
    else
      raise "Unknown write mode: #{mode}"
    end
  end

  def validate_output!
    output[0...-1].each_with_index do |val, i|
      raise "Bad output value at index #{i}: #{val}" unless val == 0
    end
  end
end

def run_program(program, input)
  IntcodeComputer.run(program, input)
end

def main
  program_str = File.read(INPUT_FILE)
  program = parse_program(program_str)

  log.info "Part 1:"
  log.info measure{run_program(program, [1]).output.last}

  log.info "Part 2:"
  log.info measure{run_program(program, [5]).output.last}
end

if __FILE__ == $0
  main
end