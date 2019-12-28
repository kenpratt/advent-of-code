require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

PART1_INPUT_FILE = File.join(__dir__, 'part1', 'input.txt')
PART1_EXAMPLE1_INPUT_FILE = File.join(__dir__, 'part1', 'example1.txt')
PART1_EXAMPLE2_INPUT_FILE = File.join(__dir__, 'part1', 'example2.txt')
PART1_EXAMPLE3_INPUT_FILE = File.join(__dir__, 'part1', 'example3.txt')
PART1_EXAMPLE4_INPUT_FILE = File.join(__dir__, 'part1', 'example4.txt')
PART1_EXAMPLE5_INPUT_FILE = File.join(__dir__, 'part1', 'example5.txt')

PART2_INPUT_FILE = File.join(__dir__, 'part2', 'input.txt')
PART2_EXAMPLE1_INPUT_FILE = File.join(__dir__, 'part2', 'example1.txt')
PART2_EXAMPLE2_INPUT_FILE = File.join(__dir__, 'part2', 'example2.txt')
PART2_EXAMPLE3_INPUT_FILE = File.join(__dir__, 'part2', 'example3.txt')
PART2_EXAMPLE4_INPUT_FILE = File.join(__dir__, 'part2', 'example4.txt')

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [PART1_EXAMPLE1_INPUT_FILE, 8, ['a', 'b']],
    [PART1_EXAMPLE2_INPUT_FILE, 86, ['a', 'b', 'c', 'd', 'e', 'f']],
    [PART1_EXAMPLE3_INPUT_FILE, 132, ['b', 'a', 'c', 'd', 'f', 'e', 'g']],
    [PART1_EXAMPLE4_INPUT_FILE, 136, ['a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm']],
    [PART1_EXAMPLE5_INPUT_FILE, 81, ['a', 'c', 'f', 'i', 'd', 'g', 'b', 'e', 'h']],
  ]

  def test_examples1
    EXAMPLES.each do |input_file, expected_distance, expected_key_order|
      log.debug "running #{input_file}"
      path = find_shortest_path_to_collect_all_keys(input_file)
      assert_equal(expected_distance, path.distance)
      #assert_equal(expected_key_order, path.collected_keys.to_a)
    end
  end

  def test_input1
    path = find_shortest_path_to_collect_all_keys(PART1_INPUT_FILE)
    assert_equal(6162, path.distance)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    [PART2_EXAMPLE1_INPUT_FILE, 8],
    [PART2_EXAMPLE2_INPUT_FILE, 24],
    [PART2_EXAMPLE3_INPUT_FILE, 32],
    [PART2_EXAMPLE4_INPUT_FILE, 72],
  ]

  def test_examples2
    EXAMPLES.each do |input_file, expected_distance, expected_key_order|
      log.debug "running #{input_file}"
      path = find_shortest_path_to_collect_all_keys_with_multiple_robots(input_file)
      assert_equal(expected_distance, path.distance)
      #assert_equal(expected_key_order, path.collected_keys.to_a)
    end
  end

#   def test_examples
#     EXAMPLES.each do |input_str, expected_output|
#       input = process_input(input_str)
#       res = part2(input)
#       assert_equal(expected_output, res)
#     end
#   end

#   def test_input
#     input_str = File.read(INPUT_FILE)
#     input = process_input(input_str)
#     res = part2(input)
#     assert_equal(nil, res)
#   end
end
