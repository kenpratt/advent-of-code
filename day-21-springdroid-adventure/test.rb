require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = survey_damage_with_springdroid_walking(program)
    assert_equal(19360724, res)
  end
end

class TestPart2 < Minitest::Test
  def test_input2
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)
    res = survey_damage_with_springdroid_running(program)
    assert_equal(0, res)
  end
end
