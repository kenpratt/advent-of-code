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
    #log.debug "run_phase position: #{position}"
    calculate_phase_for_position(input, position)
  end
end

def calculate_phase_for_position(input, position)
  index = position
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

def calculate_result_with_offset_output(input, input_repeat, num_phases, result_size)
  result_offset = input[0, 7].join('').to_i
  length = input.size * input_repeat

  # after halfway through positions, we can take a huge shortcut:
  # the values are just (Pos * 1) + ((Pos + 1) * 2) + ((Pos + 2) * 3)...
  if result_offset >= length / 2
    repeated_input = input * input_repeat
    subset = repeated_input[result_offset..-1]
    result = run_positive_summation_phases(subset, num_phases)
    result[0, result_size]
  else
    raise "Don't yet know how to calculate an offset for this scenario"
  end
end

def run_positive_summation_phases(arr, num_phases)
  input = arr
  output = nil
  num_phases.times do
    output = run_positive_summation_phase(input)
    input = output
  end
  output
end

def run_positive_summation_phase(arr)
  length = arr.size
  out = Array.new(length)
  acc = 0
  (length - 1).downto(0) do |i|
    acc = (acc + arr[i]) % 10
    out[i] = acc
  end
  out
end