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
  offsetted_input = repeated_input[result_offset..-1]

  log.debug "running phases with input size #{repeated_input.size}, #{num_phases} phases, result offset: #{result_offset}, result size: #{result_size}, offsetted_input size #{offsetted_input.size}"

  phase_input = offsetted_input
  phase_output = nil
  num_phases.times do |i|
    log.debug "phase #{i}"
    phase_output = run_phase(phase_input, result_offset)
    phase_input = phase_output
  end
  phase_output[0, result_size] # pre-offsetted
end

def run_phase(input, position_offset=0)
  input.each_index.map do |index|
    log.debug "run_phase position: #{index} #{position_offset}" if index % 1000 == 0
    calculate_phase_for_position(input, index, position_offset)
  end
end

def calculate_phase_for_position(input, index, position_offset)
  position = index + position_offset
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

