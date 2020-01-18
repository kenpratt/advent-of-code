require_relative 'instructions'

class IntcodeComputer
  attr_reader :memory, :instruction_pointer, :relative_base, :input, :output, :name, :cycles

  def initialize(program, input, name='Computer')
    raise "No program provided" if program.nil? || program.size == 0 
    @memory = program.clone
    @instruction_pointer = 0
    @relative_base = 0
    @input = input.clone
    @output = []
    @blocked = false
    @halted = false
    @name = name
    @cycles = 0
  end

  def self.run(program, input=[])
    computer = IntcodeComputer.new(program, input)
    computer.run
    computer
  end

  def run
    while !@blocked && !@halted
      execute_current_instruction
    end
    self
  end

  def blocked?; @blocked; end
  def halted?; @halted; end

  def execute_current_instruction
    return if @blocked
    return if @halted

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

      @cycles += 1
    end
  end

  def read(offset, mode)
    contents = direct_read(@instruction_pointer + offset)

    case mode
    when 0
      # position mode (indirect)
      pos = contents
      direct_read(pos)
    when 1
      # immediate mode (direct)
      contents
    when 2
      # relative mode (indirect with relative base)
      pos = @relative_base + contents
      direct_read(pos)
    else
      raise "Unknown read mode: #{mode}"
    end
  end

  def write(offset, mode, value)
    contents = direct_read(@instruction_pointer + offset)

    case mode
    when 0
      # position mode (indirect)
      pos = contents
      direct_write(pos, value)
    when 1
      raise "Cannot use immediate mode for writes"
    when 2
      # relative mode (indirect with relative base)
      pos = @relative_base + contents
      direct_write(pos, value)
    else
      raise "Unknown write mode: #{mode}"
    end
  end

  def direct_read(address)
    raise "Cannot read from negative address: #{address}" if address < 0
    result = @memory[address]
    # pretend we have infinite memory, and assume zero for uninitialized
    result.nil? ? 0 : result
  end

  def direct_write(address, value)
    raise "Cannot write to negative address: #{address}" if address < 0
    @memory[address] = value
  end

  def set_instruction_pointer(value)
    @instruction_pointer = value
  end

  def adjust_relative_base(value)
    @relative_base += value
  end

  def halt
    @halted = true
  end

  def add_input(input)
    @input << input
    @blocked = false
  end

  def add_input_arr(input_arr)
    @input += input_arr
    @blocked = false
  end

  def clear_output
    res = @output
    @output = []
    res
  end

  def to_s
    "[#{name}] input: #{@input.inspect}, output: #{@output.inspect}, blocked: #{@blocked}, halted: #{@halted}, ip: #{@instruction_pointer}"
  end
end