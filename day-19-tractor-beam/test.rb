require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = explore_tractor_beam(program)
    assert_equal(0, res.size)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples2
    EXAMPLES.each do |input_str, expected_output|
      program = parse_program(input_str)
      res = run_program(program)
      assert_equal(expected_output, res)
    end
  end

  def test_input2
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = run_program(program)
    assert_equal(nil, res)
  end
end
