require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'droid'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end
