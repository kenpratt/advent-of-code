require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    droid = Droid.start(program)
    binding.pry
    #res = find_password(program)
    #assert_equal(19360724, res)
  end
end