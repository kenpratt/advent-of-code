require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'
require_relative '../utils/grid'

require_relative 'computer'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def modify_program(input, noun, verb)
  out = input.clone
  out[1] = noun
  out[2] = verb
  out  
end

def run_program(program, input)
  IntcodeComputer.run(program, input)
end

class Robot
  attr_reader :position

  def initialize(position, orientation)
    @position = position
    @orientation = orientation
  end

  def turn!(direction_to_turn)
    case direction_to_turn
    when 0
      turn_left!
    when 1
      turn_right!
    else
      raise "Unknown turn instruction: #{direction_to_turn}"
    end
  end

  def turn_left!
    next_orientation = case @orientation
    when :up
      :left
    when :left
      :down
    when :down
      :right
    when :right
      :up
    else
      raise "Unknown orientation: #{@orientation}"
    end
    @orientation = next_orientation
  end

  def turn_right!
    next_orientation = case @orientation
    when :up
      :right
    when :right
      :down
    when :down
      :left
    when :left
      :up
    else
      raise "Unknown orientation: #{@orientation}"
    end
    @orientation = next_orientation
  end

  def advance!
    @position.move!(@orientation, 1)
  end

  def render_orientation
    case @orientation
    when :up
      '^'
    when :left
      '<'
    when :down
      'v'
    when :right
      '>'
    else
      raise "Unknown orientation: #{@orientation}"
    end   
  end
end

class PaintedGrid
  def initialize
    @robot = Robot.new(Coordinate.new(0, 0), :up)
    @painted = Hash.new(0)
    @bounds = Bounds.new(0, 0, 0, 0)
  end

  def current_colour
    @painted[@robot.position]
  end

  def paint!(colour)
    @painted[@robot.position.clone] = colour
    log.debug "paint #{@robot.position} #{colour}"
  end
  
  def turn_robot!(direction_to_turn)
    @robot.turn!(direction_to_turn)
    @robot.advance!
    @bounds.expand!(@robot.position)
    log.debug "turned robot! #{direction_to_turn}, now at #{@robot.position}"
  end

  def num_painted
    @painted.size
  end

  def to_s
    @bounds.render_grid do |c|
      if c == @robot.position
        @robot.render_orientation
      else
        case @painted[c]
        when 1
          '#'
        when 0
          '.'
        else
          raise "Unknown painted colour: #{@painted[c]}"
        end
      end
    end
  end
end

def run_painting_robot(program, initial_panel_colour)
  computer = IntcodeComputer.run(program, [])
  grid = PaintedGrid.new
  grid.paint!(initial_panel_colour)

  while !computer.halted?
    # read current colour from grid & input into computer
    input = grid.current_colour
    computer.add_input(input)

    computer.run

    output = computer.clear_output
    raise "Expected two outputs but received #{output}" unless output.size == 2
    colour_to_paint = output[0]
    direction_to_turn = output[1]

    log.debug "paint: #{colour_to_paint}, turn: #{direction_to_turn}"
    grid.paint!(colour_to_paint)
    grid.turn_robot!(direction_to_turn)
  end

  grid
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