require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'map'
require_relative 'solver'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  Map.new(input_str)
end

def find_shortest_path_to_collect_all_keys(map)
  Solver.run(map)
end

if __FILE__ == $0
  main
end