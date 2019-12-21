require_relative '../utils/log'
require_relative '../utils/system'

require_relative 'computer'
require_relative 'grid'

class Map
  attr_reader :screen

  def initialize(program)
    @program = program
    @screen = GrowableGrid.new
    @origin = Coordinate.new(0, 0)
    @droid = Droid.new(@program, @screen, @origin)
    @fully_explored = false
  end

  def explore!
    i = 0
    while !@fully_explored
      log.info "Iteration: #{i}"
      tick!
      i += 1
    end
  end

  def tick!
    destination, was_on_grid = determine_cell_to_explore
    path = calculate_path_between(@droid.position, destination)
    if path
      @droid.follow_path!(path) {render}
    else
      log.debug "found unreachable cell #{destination}, #{was_on_grid}"
      if !was_on_grid && @screen.fully_painted?
        # if the screen is fully painted *and* an off-grid cell is unreachable,
        # then we must be done
        @fully_explored = true
      else
        @screen.paint!(destination, :unreachable)
        render
      end
    end
  end

  def calculate_path_between(start, destination)
    start.path_to(destination) {|c| can_move_to_cell?(c)}
  end

  def determine_cell_to_explore
    unknown_cells = @screen.cells_with_value(nil)
    if unknown_cells.any?
      cell = unknown_cells.min {|c| c.manhattan_distance(@droid.position)}
      log.debug "found unexplored cell on grid: #{cell}"
      [cell, true]
    else
      # go off grid!
      cell = @screen.top_left_corner.move_up(1)
      log.debug "going off grid to #{cell}"
      [cell, false]
    end
  end

  def path_to_oxygen_system
    oxygen_system_cells = @screen.cells_with_value(:oxygen_system)
    return nil unless oxygen_system_cells.size == 1
    calculate_path_between(@origin, oxygen_system_cells.first)
  end

  def path_to_furthest_point_from_oxygen_system
    oxygen_system_cells = @screen.cells_with_value(:oxygen_system)
    return nil unless oxygen_system_cells.size == 1
    oxygen_system = oxygen_system_cells.first

    to_fill = @screen.cells_with_value(:empty)
    fill_paths = to_fill.map {|c| calculate_path_between(oxygen_system, c)}
    fill_paths.max_by {|p| p.size}
  end

  def render
    puts "\n"
    puts @screen.to_s {|c, v| c == @droid.position ? 'D' : value_to_s(v)}
    #sleep 0.1
  end

  def can_move_to_cell?(cell)
    @screen.value(cell) != :wall
  end

  def value_to_s(val)
    case val
    when :wall
      '#'
    when :empty
      '.'
    when :oxygen_system
      'O'
    when :unreachable
      'X'
    when nil
      ' '
    end
  end
end

class Droid
  attr_reader :computer, :position

  def initialize(program, screen, starting_position)
    @computer = IntcodeComputer.new(program, [])
    @screen = screen
    @position = starting_position
    @screen.paint!(@position, :empty)
  end

  def step!(direction)
    instruction = case direction
    when :up
      1
    when :down
      2
    when :left
      3
    when :right
      4
    else
      raise "Unknown direction: #{direction}"
    end

    requested_position = @position.move(direction, 1)

    log.debug "Running computer with input: #{instruction}"
    @computer.add_input(instruction)
    @computer.run

    output = @computer.clear_output
    log.debug "Got output: #{output}"
    raise "expected output of size 1" unless output.size == 1
    result = output.first
    case result
    when 0
      @screen.paint!(requested_position, :wall)
      false
    when 1
      @screen.paint!(requested_position, :empty)
      @position = requested_position
      true
    when 2
      @screen.paint!(requested_position, :oxygen_system)
      @position = requested_position
      true
    else
      raise "Unknown droid program output #{result}"
    end
  end

  def follow_path!(path)
    path.each do |direction|
      moved = step!(direction)
      yield if block_given?
      return unless moved
    end
  end
end