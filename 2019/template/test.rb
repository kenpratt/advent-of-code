require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples1
    EXAMPLES.each do |input_str, expected_output|
      input = process_input(input_str)
      res = part1(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input1
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    res = part1(input)
    assert_equal(nil, res)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples2
    EXAMPLES.each do |input_str, expected_output|
      input = process_input(input_str)
      res = part2(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input2
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    res = part2(input)
    assert_equal(nil, res)
  end
end
