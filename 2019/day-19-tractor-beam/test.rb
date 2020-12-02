require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = explore_tractor_beam(program)
    assert_equal(0, res.size)
  end
end

class TestPart2 < Minitest::Test
  def test_example2
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = find_point_where_ship_fits_in_tractor_beam(program, 10)
    assert_equal(990038, res.x * 10000 + res.y)
  end

  def test_input2
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = find_point_where_ship_fits_in_tractor_beam(program, 100)
    assert_equal(10730411, res.x * 10000 + res.y)
  end
end
