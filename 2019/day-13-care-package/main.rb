require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'game'

INPUT_FILE = File.join(__dir__, 'input.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def run_game(program, game_mode=nil)
  game = Game.new(program, game_mode)
  #game.run_interactive
  game.run_with_ai
  game
end

def main
  program_str = File.read(INPUT_FILE)
  program = parse_program(program_str)
  run_game(program, 2)
end

if __FILE__ == $0
  main
end