require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(raw_input)
  raw_input.strip.split(',').map(&:to_i)
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

def main
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