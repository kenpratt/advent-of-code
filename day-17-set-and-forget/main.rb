require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'
require_relative 'grid'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def start_program(program)
  computer = IntcodeComputer.new(program, [])
  computer.run
end

def sum_alignment_parameters(screen_contents_str)
  screen = Screen.new(screen_contents_str)
  intersections = screen.scaffold_intersections
  intersections.map {|c| c.x * c.y}.sum
end

class Screen
  attr_reader :grid

  def initialize(contents_str)
    rows = contents_str.split("\n").map {|s| s.each_char.to_a}
    @grid = StaticGrid.from_rows(rows)
  end

  def scaffold_intersections
    @grid.cells_with_value('#').select do |coord|
      coord.neighbours.all? {|_, n| @grid.value(n) == '#'}
    end
  end
end
