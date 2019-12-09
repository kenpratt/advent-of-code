require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  raw_input.strip.split(',').map(&:to_i)
end

def modify_input(input, noun, verb)
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
      execute(inst)
      @instruction_pointer += inst.size
    end
    validate_output!
  end

  def execute(inst)
    case inst.opcode
    when 1
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      write(3, inst.param3_mode, val1 + val2)
    when 2
      val1 = read(1, inst.param1_mode)
      val2 = read(2, inst.param2_mode)
      write(3, inst.param3_mode, val1 * val2)
    when 3
      val = @input.shift
      write(1, inst.param1_mode, val)
    when 4
      @output << read(1, inst.param1_mode)
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

def part1(program)
  IntcodeComputer.run(program, [1])
end

def part2(input)
  nil
end

def main
  raw_input = File.read(INPUT_FILE)
  input = process_input(raw_input)

  log.info "Part 1:"
  log.info measure{part1(input).output.last}

  log.info "Part 2:"
  log.info measure{part2(input)}
end

if __FILE__ == $0
  main
end