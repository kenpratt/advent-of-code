require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

class TestDay15 < Minitest::Test
  PART1_EXAMPLE = [
    "..#..........\n..#..........\n#######...###\n#.#...#...#.#\n#############\n..#...#...#..\n..#####...^..",
    76,
  ]

  def test_part1_example
    screen_str, expected_output = *PART1_EXAMPLE
    sum_alignment_parameters(screen_str)
  end

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    computer = start_program(program)
    output = computer.clear_output
    screen_str = output.map(&:chr).join('')
    sum = sum_alignment_parameters(screen_str)
    assert_equal(11140, sum)
  end

  # def test_part_2
  #   program_str = File.read(INPUT_FILE)
  #   program = parse_program(program_str)
  #   path = find_furthest_point_from_oxygen_system(program)
  #   assert_equal(376, path.size)
  # end  
end
