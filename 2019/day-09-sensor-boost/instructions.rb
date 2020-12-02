class Instruction
  def self.parse(op, computer)
    opcode = op % 100
    param_modes = op / 100
    klass = class_for_opcode(opcode)
    klass.new(param_modes, computer)
  end

  def self.class_for_opcode(opcode)
    @opcode_lookup_table ||= build_opcode_lookup_table
    if !@opcode_lookup_table.has_key?(opcode)
      raise "Opcode not found: #{opcode}"
    end
    @opcode_lookup_table[opcode]
  end

  def self.build_opcode_lookup_table
    instructions = ObjectSpace.each_object(Class).select {|klass| klass < self}
    instructions.map {|i| [i.opcode, i]}.to_h
  end

  def initialize(param_modes, computer)
    @param_modes = param_modes
    @computer = computer
  end

  def param_count
    self.class.param_count
  end

  def size
    param_count + 1
  end

  def read(offset)
    mode = param_mode(offset)
    @computer.read(offset, mode)
  end

  def write(offset, val)
    mode = param_mode(offset)
    @computer.write(offset, mode, val)
  end

  def set_instruction_pointer(val)
    @computer.set_instruction_pointer(val)
  end

  def adjust_relative_base(val)
    @computer.adjust_relative_base(val)
  end

  def input; @computer.input; end
  def output; @computer.output; end
  def halt; @computer.halt; end

  def param_mode(offset)
    case offset
    when 1
      @param_modes % 10
    when 2
      (@param_modes / 10) % 10
    when 3
      (@param_modes / 100) % 10
    else
      raise "Unknown offset: #{offset}"
    end
  end

  # optionally override
  def blocked?
    return false
  end
end

class Add < Instruction
  def self.opcode; 1; end
  def self.param_count; 3; end
  def execute
    # add (v1 + v2)
    v1 = read(1)
    v2 = read(2)
    write(3, v1 + v2)
  end
end

class Multiply < Instruction
  def self.opcode; 2; end
  def self.param_count; 3; end
  def execute
    # multiply (v1 + v2)
    v1 = read(1)
    v2 = read(2)
    write(3, v1 * v2)
  end
end

class ReadInput < Instruction
  def self.opcode; 3; end
  def self.param_count; 1; end
  def blocked?; input.empty?; end
  def execute
    log.debug "[#{@computer.name}] read input: #{input.inspect}"
    # takes a single integer as input and saves it to the position given by its only parameter.
    val = input.shift
    write(1, val)
  end
end

class WriteOutput < Instruction
  def self.opcode; 4; end
  def self.param_count; 1; end
  def execute
    log.debug "[#{@computer.name}] write output: #{input.inspect}"
    # outputs the value of its only parameter.
    output << read(1)
  end
end

class JumpIfTrue < Instruction
  def self.opcode; 5; end
  def self.param_count; 2; end
  def execute
    # jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value
    # from the second parameter. Otherwise, it does nothing.
    val1 = read(1)
    if val1 != 0
      val2 = read(2)
      set_instruction_pointer(val2)
    end
  end
end

class JumpIfFalse < Instruction
  def self.opcode; 6; end
  def self.param_count; 2; end
  def execute
    # jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value
    # from the second parameter. Otherwise, it does nothing.
    val1 = read(1)
    if val1 == 0
      val2 = read(2)
      set_instruction_pointer(val2)
    end
  end
end

class LessThan < Instruction
  def self.opcode; 7; end
  def self.param_count; 3; end
  def execute
    # less than: if the first parameter is less than the second parameter, it stores 1 in the position
    # given by the third parameter. Otherwise, it stores 0.
    val1 = read(1)
    val2 = read(2)
    res = val1 < val2 ? 1 : 0
    write(3, res)
  end
end

class Equal < Instruction
  def self.opcode; 8; end
  def self.param_count; 3; end
  def execute
    # equals: if the first parameter is equal to the second parameter, it stores 1 in the position
    # given by the third parameter. Otherwise, it stores 0.
    val1 = read(1)
    val2 = read(2)
    res = val1 == val2 ? 1 : 0
    write(3, res)
  end
end

class AdjustRelativeBase < Instruction
  def self.opcode; 9; end
  def self.param_count; 1; end
  def execute
    # takes a single integer as input and adjusts the relative base by that much
    val = read(1)
    adjust_relative_base(val)
  end
end

class Halt < Instruction
  def self.opcode; 99; end
  def self.param_count; 0; end
  def execute
    halt
  end
end