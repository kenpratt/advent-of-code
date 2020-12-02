require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  lines = input_str.split("\n")

  asteroids = []
  starting_position = nil

  lines.each_with_index do |line, y|
    line.each_char.each_with_index do |char, x|
      case char
      when '#'
        asteroids << [x, y]
      when 'X'
        starting_position = [x, y]
      when '.'
        nil
      else
        raise "Unknown char: #{char}"
      end
    end
  end

  [asteroids, starting_position]
end

def counts_per_asteroid(asteroids)
  asteroids.each_with_index.map do |asteroid, index|
    other_asteroids = asteroids[0...index] + asteroids[(index + 1)..-1]
    count = group_by_angle(asteroid, other_asteroids).size
    [asteroid, count]
  end.to_h
end

STARTING_ANGLE = Math.atan2(-1, 0)

def vaporize_asteroids(starting_position, asteroids)
  # group by angle, sorted by nearest first per angle
  by_angle = group_by_angle(starting_position, asteroids)
  by_angle.each do |angle, asteroids_for_angle|
    asteroids_for_angle.sort_by! {|asteroid| manhattan_distance(starting_position, asteroid)}
  end

  # figure out what order to scan angles in
  angles = by_angle.keys.sort
  starting_index = angles.find_index {|a| a >= STARTING_ANGLE}
  ordered_angles = angles[starting_index..-1] + angles[0...starting_index]

  # what's the most things to vaporize at an angle?
  num_passes = by_angle.map {|angle, asteroids| asteroids.size}.max

  # do the vapourization passes
  num_passes.times.map do |nth_asteroid|
    ordered_angles.map do |angle|
      by_angle[angle][nth_asteroid]
    end.compact
  end.flatten(1)
end

def group_by_angle(starting_position, asteroids)
  asteroids.group_by {|asteroid| angle(starting_position, asteroid)}
end

def angle(point1, point2)
  dx = point2[0] - point1[0]
  dy = point2[1] - point1[1]
  Math.atan2(dy, dx)  
end

def manhattan_distance(point1, point2)
  (point2[0] - point1[0]).abs + (point2[1] - point1[1]).abs
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