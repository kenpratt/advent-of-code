require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

EXAMPLE1_INPUT_FILE = File.join(__dir__, 'example1.txt')
EXAMPLE2_INPUT_FILE = File.join(__dir__, 'example2.txt')
EXAMPLE3_INPUT_FILE = File.join(__dir__, 'example3.txt')
EXAMPLE4_INPUT_FILE = File.join(__dir__, 'example4.txt')
EXAMPLE5_INPUT_FILE = File.join(__dir__, 'example5.txt')

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [EXAMPLE1_INPUT_FILE, 8, ['a', 'b']],
    [EXAMPLE2_INPUT_FILE, 86, ['a', 'b', 'c', 'd', 'e', 'f']],
    [EXAMPLE3_INPUT_FILE, 132, ['b', 'a', 'c', 'd', 'f', 'e', 'g']],
    [EXAMPLE4_INPUT_FILE, 136, ['a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm']],
    [EXAMPLE5_INPUT_FILE, 81, ['a', 'c', 'f', 'i', 'd', 'g', 'b', 'e', 'h']],
  ]

  def test_examples
    EXAMPLES.each do |input_file, expected_distance, expected_key_order|
      log.debug "running #{input_file}"
      path = find_shortest_path_to_collect_all_keys(input_file)
      assert_equal(expected_distance, path.distance)
      #assert_equal(expected_key_order, path.collected_keys.to_a)
    end
  end

  def test_input
    path = find_shortest_path_to_collect_all_keys(INPUT_FILE)
    assert_equal(1, path.steps)
  end
end

# class TestPart2 < Minitest::Test
#   EXAMPLES = [
#   ]

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
# end
