require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'map'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def find_shortest_path_to_oxygen_system(program)
  map = explore_map(program)
  binding.pry
  0
end

def explore_map(program)
  map = Map.new(program)
  map.explore!
  map
end

def main
  program_str = File.read(INPUT_FILE)
  program = parse_program(program_str)
  run_game(program, 2)
end

if __FILE__ == $0
  main
end