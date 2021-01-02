require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_solver
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = Solver.run(program)
    assert_equal("2424308736", res)
  end
end