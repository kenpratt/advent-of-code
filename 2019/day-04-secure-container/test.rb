require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |input_str, output|
      input = process_input(input_str)
      res = part1(input)
      assert_equal(output, res)
    end
  end
end

class TestPart2 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |input_str, output|
      input = process_input(input_str)
      res = part2(input)
      assert_equal(output, res)
    end
  end
end
