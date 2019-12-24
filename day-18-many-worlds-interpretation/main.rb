require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'grid'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  rows = input_str.split("\n").map {|s| s.split('')}
  StaticGrid.from_rows(rows)
end

def find_shortest_paths_to_collect_all_keys(map)
  binding.pry
end

if __FILE__ == $0
  main
end