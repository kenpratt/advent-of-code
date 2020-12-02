require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
    ['123456789012', 3, 2, [[1, 2, 3, 4, 5, 6], [7, 8, 9, 0, 1, 2]], 1],
  ]

  def test_examples
    EXAMPLES.each do |input_str, width, height, expected_layers, expected_result|
      layers = process_input(input_str, width, height)
      assert_equal(expected_layers, layers.map(&:pixels))
      result = part1(layers)
      assert_equal(expected_result, result)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    layers = process_input(input_str, 25, 6)
    result = part1(layers)
    assert_equal(1742, result)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    ['0222112222120000', 2, 2, [0, 1, 1, 0]],
  ]

  def test_examples
    EXAMPLES.each do |input_str, width, height, expected_output|
      layers = process_input(input_str, width, height)
      result = flatten_layers(layers)
      assert_equal(expected_output, result.pixels)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    layers = process_input(input_str, 25, 6)
    layer = flatten_layers(layers)
    puts "Part 2"
    puts layer
    puts
  end
end
