require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'solver'
require_relative 'recursive_solver'

def find_shortest_path_to_exit(map_file)
  Solver.run(map_file)
end

def find_shortest_path_to_exit_in_recursive_maze(map_file)
  RecursiveSolver.run(map_file)
end