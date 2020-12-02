require 'minitest/autorun'

require_relative './main'

log.level = Logger::INFO

class TestDay11 < Minitest::Test
  INPUT_FILE = File.join(__dir__, '..', 'day-11-space-police', 'input.txt')

  EXAMPLES = [
    [
      '3,43,104,1,104,0,3,43,104,0,104,0,3,43,104,1,104,0,3,43,104,1,104,0,3,43,104,0,104,1,3,43,104,1,104,0,3,43,104,1,104,0,99,0',
      ".<#\n..#\n##.",
      6,
    ],
  ]

  def test_examples
    EXAMPLES.each do |program_str, expected_grid, expected_num_painted|
      program = parse_program(program_str)
      grid = run_painting_robot(program, 0)
      assert_equal(expected_grid, grid.to_s)
      assert_equal(expected_num_painted, grid.num_painted)
    end
  end

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    grid = run_painting_robot(program, 0)
    assert_equal(2018, grid.num_painted)
  end

  def test_part_2
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    grid = run_painting_robot(program, 1)
    assert_equal(249, grid.num_painted)
    puts "Part 2:"
    puts grid
    puts
  end
end
