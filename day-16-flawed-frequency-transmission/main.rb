require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

def run_phases_with_output_offset(input, input_repeat, num_phases, result_size)
  result_offset = input[0, 7].join('').to_i
  repeated_input = input * input_repeat

  log.debug "running phases with input size #{repeated_input.size}, #{num_phases} phases, result offset: #{result_offset}, result size: #{result_size}"
  output = nil
  num_phases.times do |i|
    log.debug "phase #{i}"
    output = run_phase(repeated_input)
    input = output
  end
  output[result_offset, result_size]
end

def run_phase(input, stop_at=nil)
  input.each_index.map do |position|
    next if stop_at && position >= stop_at
    #log.debug "run_phase position: #{position}"
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
    
    i = index
    stop = index + num_repeats
    stop = length if stop > length
    while i < stop
      sum += input[i]
      i += 1
    end

    negative_values_index = index + negative_values_offset
    i = negative_values_index
    stop = negative_values_index + num_repeats
    stop = length if stop > length
    while i < stop
      sum -= input[i]
      i += 1
    end    

    index += chunk_size
  end

  sum.abs % 10
end

