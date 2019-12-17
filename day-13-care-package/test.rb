require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestDay13 < Minitest::Test
  INPUT_FILE = File.join(__dir__, '..', 'day-13-care-package', 'input.txt')

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    game = run_game(program)
    assert_equal(true, game.computer.halted?)

    num_blocks = game.screen.cells.count {|c, v| v == 2}
    assert_equal(265, num_blocks)
  end
end
