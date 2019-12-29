require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

INPUT_FILE = File.join(__dir__, 'input.txt')

PART1_EXAMPLE1_INPUT_FILE = File.join(__dir__, 'part1', 'example1.txt')
PART1_EXAMPLE2_INPUT_FILE = File.join(__dir__, 'part1', 'example2.txt')

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
