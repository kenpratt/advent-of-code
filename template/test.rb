require 'minitest/autorun'

load File.join(__dir__, 'main.rb')
log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = part1(input)
      assert_equal(output, res)
    end
  end
end

class TestPart2 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = part2(input)
      assert_equal(output, res)
    end
  end
end
