require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'solver'

def find_shortest_path_to_exit(map_file)
  Solver.run(map_file)
end