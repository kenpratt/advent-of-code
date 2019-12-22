require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

PATTERN = [0, 1, 0, -1]
PATTERN_SIZE = PATTERN.size

def warm_cache(input_size)
  log.debug 'warming cache'
  $pattern_values = Array.new(input_size)
  (0...input_size).each do |position|
    log.debug "warming cache for position #{position}"
    inner = Array.new(input_size)
    (0...input_size).each do |index|
      inner[index] = calculate_pattern_value(position, index)
    end
    $pattern_values[position] = inner
  end
  log.debug 'warmed cache'
end

def calculate_pattern_value(position, index)
  pattern_index = ((index + 1) / (position + 1)) % PATTERN_SIZE
  PATTERN[pattern_index]
end

def run_phase(input)
  warm_cache(input.size) unless $pattern_values
  input.each_index.map do |position|
    #log.debug "run_phase #{position}"
    calculate_phase_for_position(input, position)
  end
end

def calculate_phase_for_position(input, position)
  #log.debug "calc: #{input.inspect} #{position.inspect}"
  pattern_values = $pattern_values[position]

  val = input.each_with_index.map do |element, i|
    multiplier = pattern_values[i]
    res = element * multiplier
    #log.debug "mult: #{element} * #{multiplier} = #{res}"
    res
  end.sum

  val.abs % 10
end
