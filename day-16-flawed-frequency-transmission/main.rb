require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

def run_phase(input, stop_at=nil)
  input.each_index.map do |position|
    next if stop_at && position >= stop_at
    #log.debug "run_phase #{position}"
    calculate_phase_for_position(input, position)
  end
end

def calculate_phase_for_position(input, position)
  index = position # skip first pos-1 elements
  length = input.size
  num_repeats = position + 1
  negative_values_offset = num_repeats * 2
  chunk_size = num_repeats * 4
  sum = 0

  while index < length
    #log.debug "length: #{length}, chunk_size: #{chunk_size}, index: #{index}"
    negative_values_index = index + negative_values_offset
    
    num_repeats.times do |offset|
      i = index + offset
      sum += input[i] if i < length
    end
    num_repeats.times do |offset|
      i = negative_values_index + offset
      sum -= input[i] if i < length
    end    
    index += chunk_size
  end

  sum.abs % 10
end

