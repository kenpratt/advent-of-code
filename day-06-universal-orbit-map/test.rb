require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
    ["COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L", 42],
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
    assert_equal(145250, res)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    ["COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN", 4],
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
    assert_equal(274, res)
  end
end
