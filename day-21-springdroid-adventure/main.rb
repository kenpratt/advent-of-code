require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def survey_damage_with_springdroid_walking(program)
  # D && (!A || !B || !C)
  springscript = [
    'NOT A J',
    'NOT B T',
    'OR T J',
    'NOT C T',
    'OR T J',
    'AND D J',
    'WALK',
  ]
  run_springdroid(program, springscript)
end

def survey_damage_with_springdroid_running(program)
  jump_analysis

  # D && (!A || !B || (!C && (!X || Y))
  # X = (!E && F)
  # Y = (!G && H && I)
  springscript = [
    # Y => J
    'NOT G J',
    'AND H J',
    'AND I J',

    # !X => T
    # !X = !(!E && F) = E || !F
    'NOT F T',
    'OR E T',

    # (!X || Y) => J
    'OR T J',

    # (!C && ...) => J
    'NOT C T',
    'AND T J',

    # !B || ... => J
    'NOT B T',
    'OR T J',

    # !A || ... => J
    'NOT A T',
    'OR T J',

    # D && (...)
    'AND D J',

    'RUN',
  ]

  run_springdroid(program, springscript)
end

def run_springdroid(program, springscript)
  input_arr = springscript.flat_map do |line|
    line.each_char.map(&:ord) + [10]
  end
  computer = IntcodeComputer.new(program, input_arr)
  computer.run
  output = computer.clear_output

  result = nil
  if output.last > 255
    result = output.pop
  end

  log.info output.map(&:chr).join('')
  log.info "Cycles: #{computer.cycles}"

  result
end

# simulate 4 tiles past lookahead, as a solution could contain a jump at
# anything from 0-9, and if we jump at 9 we'll land at 13.
JUMP_SIZE = 4
LOOKAHEAD = 9
SIMULATE_TILES = LOOKAHEAD + JUMP_SIZE

def jump_analysis
  tile_combinations = generate_combinations([[true, false]] * SIMULATE_TILES)

  possible_solutions = SortedSet.new

  possible_solutions << Solution.new([]) # no jump
  (0...LOOKAHEAD).each do |first_jump_position|
    possible_solutions << Solution.new([first_jump_position]) # single jump
    first_landing_position = first_jump_position + JUMP_SIZE
    if first_landing_position < LOOKAHEAD
      (first_landing_position...LOOKAHEAD).each do |second_jump_position|
        possible_solutions << Solution.new([first_jump_position, second_jump_position]) # double jump
        second_landing_position = second_jump_position + JUMP_SIZE
        if second_landing_position < LOOKAHEAD
          (second_landing_position...LOOKAHEAD).each do |third_jump_position|
            possible_solutions << Solution.new([first_jump_position, second_jump_position, third_jump_position]) # triple jump
          end
        end
      end
    end
  end

  alive_sols_per_combo = tile_combinations.map do |tiles|
    [tiles, possible_solutions.select {|s| s.alive?(tiles)}]
  end
  
  # filter out dead no matter what
  alive_sols_per_combo.reject! {|tiles, sols| sols.empty?}

  # filter out no holes
  #alive_sols_per_combo.reject! {|tiles, sols| tiles.all? {|t| t}}

  # filter out first tile is a hole (must jump)
  #alive_sols_per_combo.reject! {|tiles, sols| !tiles[0]}

  # alive_sols_per_combo.map {|tiles, sols| [tiles.map {|s| s ? '#' : '.'}.join(''), sols.first.jumps]}   

  with_best_soln = alive_sols_per_combo.map {|tiles, sols| [tiles, sols.first]};

  heuristic = lambda {|t| t[3] && (!t[0] || !t[1] || (!t[2] && ((t[4] || !t[5]) || (!t[6] && t[7] && t[8]))))}

  analysis = (0..LOOKAHEAD).map do |jump_offset|
    res = with_best_soln.select do |t, s|
      s.first_jump_at >= jump_offset
    end.group_by do |t, s|
      (s.first_jump_at == jump_offset) == heuristic.call(t[jump_offset..-1])
    end.to_h
    [jump_offset, res]
  end.to_h

  analysis.map {|o, x| [o, x.map {|k, v| [k, v.size]}]}

  # with_best_soln.map {|tiles, s| [tiles.map {|t| t ? '#' : '.'}.join(''), s.jumps]}   
  # with_best_soln.select {|_, s| s.first_jump_at == 0}.map {|tiles, s| [tiles.map {|t| t ? '#' : '.'}.join(''), s.jumps]}.sort_by(&:first)

  # TODO
  # want to jump at '##.#.#.##'
  # but NOT any other prefix of '##.#.#'
  # maybe there's a "would a quick double jump work?" heuristic in here,
  # along with an extra thing to avoid the unwanted jumps with a hole at 2
  # (maybe it's as simple as not another hole at 5?)

  # might be able to look for things with a hole at t[2] (and not 0..1 or 3),
  # and see if there's a pattern of when to jump

  # after I get a working solution, also see if it'll correctly work for
  # the jumps @1..9 (at least with as much info as possible)

  binding.pry
end

class Solution
  attr_reader :jumps, :num_jumps, :first_jump_at, :last_jump_at, :positions_on_ground

  def initialize(jumps)
    @jumps = jumps
    @num_jumps = jumps.size
    @first_jump_at = jumps.any? ? jumps.first : -1
    @last_jump_at = jumps.any? ? jumps.last : -1
    @positions_on_ground = calculate_positions_on_ground
  end

  def jump?
    @first_jump_at == 0
  end

  def <=>(other)
    [@last_jump_at, @num_jumps, @first_jump_at] <=> [other.last_jump_at, other.num_jumps, other.first_jump_at]
  end

  def calculate_positions_on_ground
    out = (1..SIMULATE_TILES).to_a
    @jumps.each do |jump|
      # in air for jump+1, jump+2, jump+3
      out -= ((jump + 1)...(jump + JUMP_SIZE)).to_a
    end
    out
  end

  def alive?(tiles)
    @positions_on_ground.all? do |pos|
      tiles[pos - 1]
    end
  end
end

def generate_combinations(input)
  output = []
  generate_combinations_(input, 0, [], output)
  output
end

def generate_combinations_(input, i, built, output)
  if i == input.size
    output << built
  else
    input[i].each do |v|
      generate_combinations_(input, i + 1, built + [v], output)
    end
  end
end
