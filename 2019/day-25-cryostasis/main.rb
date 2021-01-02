require 'colorize'
require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'droid'
require_relative 'solver'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end
