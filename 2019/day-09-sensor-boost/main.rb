require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def modify_program(input, noun, verb)
  out = input.clone
  out[1] = noun
  out[2] = verb
  out  
end

def run_program(program, input)
  IntcodeComputer.run(program, input)
end

def run_amplifiers(program, phase_settings, initial_input)
  # initialize computers with phase settings
  computers = phase_settings.each_with_index.map do |phase_setting, index|
    IntcodeComputer.new(program, [phase_setting], "Computer ##{index}")
  end

  # prime initial input
  computers.first.add_input(initial_input)

  # run in "parallel", with outputs hooked up to inputs
  num_computers = computers.size
  curr_index = 0
  halt = false
  while !halt
    next_index = (curr_index + 1) % num_computers
    curr_computer = computers[curr_index]
    next_computer = computers[next_index]

    # run this computer
    curr_computer.run

    # check if done
    halt = computers.all? {|c| c.halted?}

    # shift output to next computer's input
    if !halt
      while curr_computer.output.any?
        next_computer.add_input(curr_computer.output.shift)
      end
    end

    curr_index = next_index
  end

  computers.last.output.first
end

def find_best_phase_setting_permutation(program, phase_settings, initial_input)
  phase_settings.permutation.inject(nil) do |curr_best, setting_to_try|
    thruster_signal = run_amplifiers(program, setting_to_try, initial_input)
    if curr_best.nil? || thruster_signal > curr_best[0]
      [thruster_signal, setting_to_try]
    else
      curr_best
    end
  end
end

def main
  # (0..4).to_a.permutation
  program_str = File.read(INPUT_FILE)
  program = parse_program(program_str)

  log.info "Part 1:"
  log.info measure{run_program(program, [1]).output.last}

  log.info "Part 2:"
  log.info measure{run_program(program, [5]).output.last}
end

if __FILE__ == $0
  main
end