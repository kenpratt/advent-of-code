require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  lines = input_str.split("\n")

  asteroids = []
  lines.each_with_index do |line, y|
    line.each_char.each_with_index do |char, x|
      if char == '#'
        asteroids << [x, y]
      end
    end
  end
  asteroids
end

def counts_per_asteroid(asteroids)
  asteroids.each_with_index.map do |asteroid, index|
    other_asteroids = asteroids[0...index] + asteroids[(index + 1)..-1]
    count = group_by_angle(asteroid, other_asteroids).size
    [asteroid, count]
  end.to_h
end

def group_by_angle(asteroid, others)
  others.group_by {|other| angle(asteroid, other)}
end

def angle(point1, point2)
  dx = point2[0] - point1[0]
  dy = point2[1] - point1[1]
  Math.atan2(dy, dx)  
end

def best_location(counts)
  counts.max_by {|asteroid, count| count}
end

def part1(input)
  nil
end

def part2(input)
  nil
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