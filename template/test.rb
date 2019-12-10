require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples
    EXAMPLES.each do |raw_input, expected_output|
      input = process_input(raw_input)
      res = part1(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input
    raw_input = File.read(INPUT_FILE)
    input = process_input(raw_input)
    res = part1(input)
    assert_equal(nil, res)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
  ]

  def test_examples
    EXAMPLES.each do |raw_input, expected_output|
      input = process_input(raw_input)
      res = part2(input)
      assert_equal(expected_output, res)
    end
  end

  def test_input
    raw_input = File.read(INPUT_FILE)
    input = process_input(raw_input)
    res = part2(input)
    assert_equal(nil, res)
  end
end
