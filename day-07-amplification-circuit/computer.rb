require_relative 'instructions'

class IntcodeComputer
    attr_reader :memory, :instruction_pointer, :input, :output
  
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
        execute_current_instruction
      end
      validate_output!
      self
    end
  
    def execute_current_instruction
      curr_instruction_pointer = @instruction_pointer
  
      # read operation from current instruction pointer (immediate mode)
      operation = read(0, 1)
  
      # parse and execute
      instruction = Instruction.parse(operation, self)
      instruction.execute
  
      # advance instruction pointer, unless the instruction modified it
      if @instruction_pointer == curr_instruction_pointer
        @instruction_pointer += instruction.size
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
  
    def write(offset, mode, value)
      case mode
      when 0
        # position mode (indirect)
        pos = @memory[@instruction_pointer + offset]
        @memory[pos] = value
      when 1
        raise "Cannot use immedate mode for writes"
      else
        raise "Unknown write mode: #{mode}"
      end
    end
  
    def set_instruction_pointer(value)
      @instruction_pointer = value
    end
  
    def halt
      @halt = true
    end
  
    def validate_output!
      @output[0...-1].each_with_index do |val, i|
        raise "Bad output value at index #{i}: #{val}" unless val == 0
      end
    end
  end