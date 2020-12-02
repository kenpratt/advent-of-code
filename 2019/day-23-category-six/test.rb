require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = run_network(program, 50)
    assert_equal([[20771, 14834]], res.buffers[255])
  end
end
