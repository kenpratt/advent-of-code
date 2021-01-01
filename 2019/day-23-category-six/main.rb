require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'network'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def run_network(program, size, port_monitor)
  Network.run(program, size, port_monitor)
end
