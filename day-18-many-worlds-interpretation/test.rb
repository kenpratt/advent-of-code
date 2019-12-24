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
      input_str = File.read(input_file)
      map = process_input(input_str)
      paths = find_shortest_paths_to_collect_all_keys(map)
      assert_equal([expected_distance], paths.map(&:distance).uniq)
      assert_equal(true, paths.any? {|p| p.key_order == expected_key_order})
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    map = process_input(input_str)
    paths = find_shortest_paths_to_collect_all_keys(map)
    expected_distance = 0 # TODO
    expected_key_order = ['a'] # TODO
    assert_equal([expected_distance], paths.map(&:distance).uniq)
    assert_equal(true, paths.any? {|p| p.key_order == expected_key_order})
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
