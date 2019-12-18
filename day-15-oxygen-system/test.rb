require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestDay15 < Minitest::Test
  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    path_length = find_shortest_path_to_oxygen_system(program)
    assert_equal(100, path_length)
  end
end
