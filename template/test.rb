require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples
    EXAMPLES.each do |input_str, expected_output|
      input = process_input(input_str)
      res = part1(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    res = part1(input)
    assert_equal(nil, res)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples
    EXAMPLES.each do |input_str, expected_output|
      input = process_input(input_str)
      res = part2(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    res = part2(input)
    assert_equal(nil, res)
  end
end
