require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

INPUT_FILE = File.join(__dir__, 'input.txt')

PART1_EXAMPLE1_INPUT_FILE = File.join(__dir__, 'part1', 'example1.txt')
PART1_EXAMPLE2_INPUT_FILE = File.join(__dir__, 'part1', 'example2.txt')

PART2_EXAMPLE1_INPUT_FILE = File.join(__dir__, 'part2', 'example1.txt')

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [PART1_EXAMPLE1_INPUT_FILE, 23],
    [PART1_EXAMPLE2_INPUT_FILE, 58],
  ]

  def test_examples1
    EXAMPLES.each do |input_file, expected_distance|
      path = find_shortest_path_to_exit(input_file)
      assert_equal(expected_distance, path.distance)
    end
  end

  def test_input1
    path = find_shortest_path_to_exit(INPUT_FILE)
    assert_equal(1, path.distance)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    [PART2_EXAMPLE1_INPUT_FILE, 396],
  ]

  def test_examples2
    EXAMPLES.each do |input_file, expected_distance|
      path = find_shortest_path_to_exit_in_recursive_maze(input_file)
      assert_equal(expected_distance, path.distance)
    end
  end

  def test_input2
    path = find_shortest_path_to_exit_in_recursive_maze(INPUT_FILE)
    assert_equal(7506, path.distance)
  end
end

