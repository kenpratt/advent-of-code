require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

EXAMPLE1_INPUT_FILE = File.join(__dir__, 'example1_input.txt')
EXAMPLE2_INPUT_FILE = File.join(__dir__, 'example2_input.txt')
EXAMPLE3_INPUT_FILE = File.join(__dir__, 'example3_input.txt')
EXAMPLE4_INPUT_FILE = File.join(__dir__, 'example4_input.txt')
EXAMPLE5_INPUT_FILE = File.join(__dir__, 'example5_input.txt')

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [EXAMPLE1_INPUT_FILE, 31],
    [EXAMPLE2_INPUT_FILE, 165],
    [EXAMPLE3_INPUT_FILE, 13312],
    [EXAMPLE4_INPUT_FILE, 180697],
    [EXAMPLE5_INPUT_FILE, 2210736],
  ]

  def test_examples
    EXAMPLES.each do |input_file, expected_output|
      log.debug("")
      input_str = File.read(input_file)
      recipes = process_input(input_str)
      ore = calculate_ore_necessary_for_one_fuel(recipes)
      assert_equal(expected_output, ore)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    recipes = process_input(input_str)
    ore = calculate_ore_necessary_for_one_fuel(recipes)
    assert_equal(443537, ore)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    [EXAMPLE3_INPUT_FILE, 82892753],
    [EXAMPLE4_INPUT_FILE, 5586022],
    [EXAMPLE5_INPUT_FILE, 460664],
  ]

  def test_examples
    EXAMPLES.each do |input_file, expected_output|
      log.debug("")
      input_str = File.read(input_file)
      recipes = process_input(input_str)
      fuel = calculate_max_fuel_for_ore(recipes, 1_000_000_000_000)
      assert_equal(expected_output, fuel)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    recipes = process_input(input_str)
    fuel = calculate_max_fuel_for_ore(recipes, 1_000_000_000_000)
    assert_equal(2910558, fuel)
  end
end
