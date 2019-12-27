require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'map'
require_relative 'solver'

INPUT_FILE = File.join(__dir__, 'input.txt')

def find_shortest_path_to_collect_all_keys(map_file)
  Solver.run(map_file)
end

if __FILE__ == $0
  main
end