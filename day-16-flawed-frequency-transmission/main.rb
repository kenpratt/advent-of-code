require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

PATTERN = [0, 1, 0, -1]
$pattern_cache = {}

def get_pattern(position)
  $pattern_cache[position] ||= PATTERN.flat_map do |v|
    [v] * (position + 1)
  end
end

def get_pattern_value(pattern, pattern_index)
  i = pattern_index % pattern.size
  pattern[i]
end

def run_phase(input)
  input.each_index.map do |i|
    calculate_phase_for_position(input, i)
  end
end

def calculate_phase_for_position(input, position)
  pattern = get_pattern(position)
  #log.debug "calc: #{input.inspect} #{position.inspect} #{pattern.inspect}"

  val = input.each_with_index.map do |element, i|
    multiplier = get_pattern_value(pattern, i + 1)
    res = element * multiplier
    #log.debug "mult: #{element} * #{multiplier} = #{res}"
    res
  end.sum

  val.abs % 10
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