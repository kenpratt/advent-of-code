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
  phase_settings.inject(initial_input) do |input, phase_setting|
    computer = run_program(program, [phase_setting, input])
    computer.output.last
  end
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