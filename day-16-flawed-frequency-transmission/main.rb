require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

PATTERN = [0, 1, 0, -1]
PATTERN_SIZE = PATTERN.size

def calculate_pattern_value(position, index)
  pattern_index = ((index + 1) / (position + 1)) % PATTERN_SIZE
  PATTERN[pattern_index]
end

def run_phase(input)
  input.each_index.map do |position|
    next if position > 9
    log.debug "run_phase #{position}"
    calculate_phase_for_position(input, position)
  end
end

def calculate_phase_for_position(input, position)
  #log.debug "calc: #{input.inspect} #{position.inspect}"
  val = 0
  input.each_with_index do |element, i|
    multiplier = calculate_pattern_value(position, i)
    res = element * multiplier
    #log.debug "mult: #{element} * #{multiplier} = #{res}"
    val += res
  end
  val.abs % 10
end
