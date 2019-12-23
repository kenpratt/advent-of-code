require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.each_char.map(&:to_i)
end

PATTERN = [0, 1, 0, -1]
PATTERN_SIZE = PATTERN.size

def calculate_result_with_offset_output(input, input_repeat, num_phases, result_size)
  result_offset = input[0, 7].join('').to_i

  log.debug("running phases with input size #{input.size}, repeated #{input_repeat} times, "\
    "#{num_phases} phases, result offset: #{result_offset}, result size: #{result_size}")

  calculate_dependency_graph(input.size * input_repeat, num_phases, result_offset, result_size)
  binding.pry

  output = nil
  num_phases.times do |i|
    log.debug "phase #{i}"
    phase_output = run_phase(phase_input, result_offset)
    phase_input = phase_output
  end
  phase_output[0, result_size] # pre-offsetted
end

def calculate_simple_sum_phase(arr)
  length = arr.size
  out = Array.new(length)
  acc = 0
  (length - 1).downto(0) do |i|
    acc = (acc + arr[i]) % 10
    out[i] = acc
  end
  out
end

def calculate_dependency_graph(length, num_phases, result_offset, result_size)
  result_positions = result_offset.upto(result_offset + result_size - 1).to_a

  log.debug "result_positions: #{result_positions.inspect}"

  result_positions.each do |result_position|
    log.debug "calculating result position: #{result_position.inspect}"
    
    num_phases.downto(1).each do |phase|
      components = calculate_components_of_position(result_position, length)

    end
  end

  # needed_positions

  # num_phases.downto(1).each do |phase|
  #   log.debug "phase #{phase}"
  #   dependencies = calculate_components_of_phase(needed_positions, length)
  #   binding.pry
  #   # TODO store dependencies
  #   needed_positions = dependencies
  # end

  graph
end

# def calculate_components_of_phase(positions, length)
#   components = calculate_components_of_position(positions.first, length)
#   binding.pry
# end

def run_phase(input, stop_at=nil)
  input.each_index.map do |position|
    next if stop_at && position >= stop_at
    #log.debug "run_phase position: #{position}"
    calculate_phase_for_position(input, position)
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




Component = Struct.new(:start, :stop, :multiple) do
  
end

def calculate_components_of_position(position, length)
  index = position # skip first pos-1 elements
  num_repeats = position + 1
  negative_values_offset = num_repeats * 2
  chunk_size = num_repeats * 4

  dependencies = []

  while index < length
    stop = index + num_repeats
    stop = length if stop > length
    dependencies << Component.new(index, stop, 1)
    index += chunk_size
  end

  index = position + negative_values_offset
  while index < length
    stop = index + num_repeats
    stop = length if stop > length
    dependencies << Component.new(index, stop, -1)
    index += chunk_size
  end

  dependencies
end