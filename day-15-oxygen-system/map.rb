require_relative '../utils/log'
require_relative '../utils/system'

require_relative 'computer'
require_relative 'grid'

class Map
  attr_reader :screen

  def initialize(program)
    @program = program
    @screen = GrowableGrid.new
  end

  def explore!
    droid = Droid.new(@program, @screen)
    loop do
      droid.move!(:left)
      render(droid.position)
      binding.pry
    end
  end

  def render(droid_position)
    puts @screen.to_s {|c, v| c == droid_position ? 'D' : value_to_s(v)}
  end

  def value_to_s(val)
    case val
    when :wall
      '#'
    when :empty
      '.'
    when :oxygen_system
      'O'
    when nil
      ' '
    end
  end
end

class Droid
  attr_reader :computer, :position

  def initialize(program, screen)
    @computer = IntcodeComputer.new(program, [])
    @screen = screen
    @position = Coordinate.new(0, 0)
    @screen.paint!(@position, :empty)
  end

  def move!(direction)
    instruction = case direction
    when :up
      1
    when :down
      2
    when :left
      3
    when :right
      4
    else
      raise "Unknown direction: #{direction}"
    end

    requested_position = @position.move(direction, 1)

    log.debug "Running computer with input: #{instruction}"
    @computer.add_input(instruction)
    @computer.run

    output = @computer.clear_output
    log.debug "Got output: #{output}"
    raise "expected output of size 1" unless output.size == 1
    result = output.first
    case result
    when 0
      @screen.paint!(requested_position, :wall)
    when 1
      @screen.paint!(requested_position, :empty)
      @position = requested_position
    when 2
      @screen.paint!(requested_position, :oxygen_system)
      @position = requested_position
    else
      raise "Unknown droid program output #{result}"
    end
  end
end