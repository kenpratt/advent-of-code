require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def survey_damage_with_springdroid_walking(program)
  # D && (!A || !B || !C)
  springscript = [
    'NOT A J',
    'NOT B T',
    'OR T J',
    'NOT C T',
    'OR T J',
    'AND D J',
    'WALK',
  ]
  run_springdroid(program, springscript)
end

def survey_damage_with_springdroid_running(program)
  # D && (!A || !B || !C)
  springscript = [
    'NOT A J',
    'NOT B T',
    'OR T J',
    'NOT C T',
    'OR T J',
    'AND D J',
    'RUN',
  ]
  run_springdroid(program, springscript)
end

def run_springdroid(program, springscript)
  input_arr = springscript.flat_map do |line|
    line.each_char.map(&:ord) + [10]
  end
  computer = IntcodeComputer.new(program, input_arr)
  computer.run
  output = computer.clear_output

  result = nil
  if output.last > 255
    result = output.pop
  end

  log.info output.map(&:chr).join('')
  log.info "Cycles: #{computer.cycles}"

  result
end


