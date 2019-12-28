require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

Coordinate = Struct.new(:x, :y)

def explore_tractor_beam(program)
    tractor_beam_coords = []

  (0..49).each do |x|
    (0..49).each do |y|
      c = Coordinate.new(x, y)
      output = check_coordinate(program, c)
      tractor_beam_coords << c if output == 1
    end
  end

  tractor_beam_coords
end

def check_coordinate(program, c)
  computer = IntcodeComputer.new(program, c.to_a)
  computer.run
  output = computer.clear_output
  log.debug "output: #{output}"
  raise "Unexpected output: #{output.inspect}" unless output.size == 1
  output[0]
end
