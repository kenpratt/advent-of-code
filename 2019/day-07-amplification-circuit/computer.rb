require_relative 'instructions'

class IntcodeComputer
    attr_reader :memory, :instruction_pointer, :input, :output, :name
  
    def initialize(program, input, name='Computer')
      @memory = program.clone
      @instruction_pointer = 0
      @input = input.clone
      @output = []
      @blocked = false
      @halted = false
      @name = name
    end
  
    def self.run(program, input=[])
      computer = IntcodeComputer.new(program, input)
      computer.run
      computer
    end
  
    def run
      @blocked = false
      @halted = false
      while !@blocked && !@halted
        execute_current_instruction
      end
      self
    end

    def blocked?; @blocked; end
    def halted?; @halted; end
  
    def execute_current_instruction
      if @instruction_pointer > @memory.size
        raise "Instruction pointer out of program memory"
      end

      curr_instruction_pointer = @instruction_pointer
  
      # read operation from current instruction pointer (immediate mode)
      operation = read(0, 1)
  
      # parse instruction
      instruction = Instruction.parse(operation, self)

      # check if blocked on input
      if instruction.blocked?
        # :(
        @blocked = true
      else
        # execute instruction
        instruction.execute
    
        # advance instruction pointer, unless the instruction modified it
        if @instruction_pointer == curr_instruction_pointer
          @instruction_pointer += instruction.size
        end
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
      @halted = true
    end

    def add_input(input)
      @input << input
      @blocked = false
    end

    def to_s
       "[#{name}] input: #{@input.inspect}, output: #{@output.inspect}, blocked: #{@blocked}, halted: #{@halted}, ip: #{@instruction_pointer}"
    end
  end