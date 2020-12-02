require_relative '../utils/log'
require_relative '../utils/system'

require_relative 'computer'
require_relative 'grid'

class Game
  attr_reader :computer, :screen, :score

  def initialize(program, game_mode)
    program = program.clone
    program[0] = game_mode if game_mode
    @computer = IntcodeComputer.new(program, [])
    @screen = GrowableGrid.new
    @score = 0
  end

  TARGET_TICK_TIME_INTERACTIVE = 0.3
  TARGET_TICK_TIME_AI = 0
  INPUT_BUFFER = 0.01 # allow 10ms buffer
  
  def run_interactive
    last_time = timestamp()

    while !@computer.halted?
      tick!

      if @computer.blocked?
        elapsed, _ = elapsed_since_timestamp(last_time)
        input_wait = TARGET_TICK_TIME_INTERACTIVE - elapsed - INPUT_BUFFER
        log.debug "waiting for input: #{input_wait}"
        input = read_input(input_wait)
        log.debug "input: #{input.inspect}"
        @computer.add_input(input)
      end

      last_time = sleep_till_next_frame(last_time, TARGET_TICK_TIME_INTERACTIVE)
    end
  end

  def run_with_ai
    last_time = timestamp()

    while !@computer.halted?
      tick!

      if @computer.blocked?
        input = use_ai_to_determine_input
        log.debug "input: #{input.inspect}"
        @computer.add_input(input)
      end

      last_time = sleep_till_next_frame(last_time, TARGET_TICK_TIME_AI)
    end
  end

  def sleep_till_next_frame(last_time, target_tick_time)
    elapsed, time = elapsed_since_timestamp(last_time)
    to_sleep = target_tick_time - elapsed
    if to_sleep > 0
      log.debug "sleeping for #{to_sleep}"
      sleep(to_sleep)
    end
    time
  end

  def tick!
    @computer.run
    output = @computer.clear_output
    paint!(output)
    render
  end

  def paint!(output)
    log.debug "drawing output: #{output.size}"
    output.each_slice(3) do |x, y, value|
      log.debug "each_slice: #{x}, #{y}, #{value}"
      if x == -1 && y == 0
        @score = value
      else      
        @screen.paint!(Coordinate.new(x, y), value)
      end
    end
  end

  def read_input(timeout)
    char = read_char_from_keyboard(timeout)
    case char
    when 'a'
      -1
    when 'e', 'd' # asdf/aoeu
      1
    else
      0
    end
  end

  def use_ai_to_determine_input
    ball_cell = @screen.cells.find {|cell, value| value == 4}[0]
    paddle_cell = @screen.cells.find {|cell, value| value == 3}[0]
    # if ball is left of paddle, input is -1, etc
    ball_cell.x <=> paddle_cell.x
  end

  def render
    puts
    puts
    puts
    puts
    puts "Score: #{@score}"
    puts
    puts screen.to_s {|v| tile_id_to_s(v)}
    puts
  end

  def tile_id_to_s(tile_id)
    case tile_id
    when 0
      # 0 is an empty tile. No game object appears in this tile.
      ' '
    when 1
      # 1 is a wall tile. Walls are indestructible barriers.
      '#'
    when 2
      # 2 is a block tile. Blocks can be broken by the ball.
      'B'
    when 3
      # 3 is a horizontal paddle tile. The paddle is indestructible.
      '_'
    when 4
      # 4 is a ball tile. The ball moves diagonally and bounces off objects.
      'o'
    else
      raise "Unknown tile id: #{tile_id}"
    end
  end
end
