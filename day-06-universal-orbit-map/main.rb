require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.split("\n").map {|line| process_orbit(line)}
end

def process_orbit(s)
  if s.strip =~ /\A([A-Z0-9]+)\)([A-Z0-9]+)\z/
    [$1, $2]
  else
    raise "Bad orbit: #{s}"
  end
end

def part1(orbit_list)
  orbit_map = build_orbit_map(orbit_list)
  calculate_orbits(orbit_map, 'COM', 0)
end

def build_orbit_map(orbit_list)
  orbits = Hash.new {|h, k| h[k] = []}
  orbit_list.each do |orbitee, orbiter|
    orbits[orbitee] << orbiter
  end
  orbits
end

def calculate_orbits(orbit_map, orbitee, depth)
  depth + orbit_map[orbitee].sum {|orbiter| calculate_orbits(orbit_map, orbiter, depth + 1)}
end

def part2(orbit_list)
  reverse_orbit_map = build_reverse_orbit_map(orbit_list)
  you_path = calculate_path(reverse_orbit_map, 'YOU', 'COM').reverse
  san_path = calculate_path(reverse_orbit_map, 'SAN', 'COM').reverse
  diverge_at = (0...you_path.size).find {|i| you_path[i] != san_path[i]}
  last_common_ancestor = diverge_at - 1

  # subtracting:
  # - last_common_ancestor to drop shared path
  # - one because we're counting orbits, not nodes
  # - one because we don't count the YOU/SAN orbits
  you_moves = you_path.size - last_common_ancestor - 1 - 1
  san_moves = san_path.size - last_common_ancestor - 1 - 1
  you_moves + san_moves
end

def build_reverse_orbit_map(orbit_list)
  orbits = {}
  orbit_list.each do |orbitee, orbiter|
    orbits[orbiter] = orbitee
  end
  orbits
end

def calculate_path(reverse_orbit_map, start_object, end_object)
  path = [start_object]
  orbiter = start_object
  while orbiter != end_object
    orbitee = reverse_orbit_map[orbiter]
    path << orbitee
    orbiter = orbitee
  end
  path
end

def main
  input_str = File.read(INPUT_FILE)
  input = process_input(input_str)

  log.info "Part 1:"
  log.info measure{part1(input)}

  log.info "Part 2:"
  log.info measure{part2(input)}
end

if __FILE__ == $0
  main
end