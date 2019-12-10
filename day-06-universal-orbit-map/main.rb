require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  raw_input.split("\n").map {|line| process_orbit(line)}
end

def process_orbit(s)
  if s.strip =~ /\A([A-Z0-9]+)\)([A-Z0-9]+)\z/
    [$1, $2]
  else
    raise "Bad orbit: #{s}"
  end
end

def part1(input)
  orbits = Hash.new {|h, k| h[k] = []}
  input.each do |orbitee, orbiter|
    orbits[orbitee] << orbiter
  end
  calculate_orbits(orbits, 'COM', 0)
end

def calculate_orbits(orbits, orbitee, depth)
  depth + orbits[orbitee].sum {|orbiter| calculate_orbits(orbits, orbiter, depth + 1)}
end

def part2(input)
  nil
end

def main
  raw_input = File.read(INPUT_FILE)
  input = process_input(raw_input)

  log.info "Part 1:"
  log.info measure{part1(input)}

  log.info "Part 2:"
  log.info measure{part2(input)}
end

if __FILE__ == $0
  main
end