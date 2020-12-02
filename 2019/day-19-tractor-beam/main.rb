require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'
require_relative 'grid'

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def check_coordinate(program, c)
  computer = IntcodeComputer.new(program, c.to_a)
  computer.run
  output = computer.clear_output
  log.debug "output: #{output}"
  raise "Unexpected output: #{output.inspect}" unless output.size == 1
  output[0]
end

def explore_tractor_beam(program)
  tractor_beam_coords = []

  (0..49).each do |x|
    (0..49).each do |y|
      c = Coordinate.new(x, y)
      output = check_coordinate(program, c)
      tractor_beam_coords << c if output == 1
    end
  end

  tractor_beam_coords
end

def find_point_where_ship_fits_in_tractor_beam(program, ship_size)
  ExploreToFitShip.new(program, ship_size).run
end

class ExploreToFitShip
  def initialize(program, ship_size)
    @program = program
    @ship_size = ship_size
    @grid = GrowableGrid.new
    @upper_bound = Coordinate.new(0, 0)
    @lower_bound = Coordinate.new(0, 0)
    @upper_bounds = []
    @lower_bounds = []
  end

  def run
    result = nil
    while result.nil?
      expand_beam(10)
      result = find_place_for_ship
    end
    result
  end

  def expand_beam(distance)
    curr_upper_bound = @upper_bound
    curr_lower_bound = @lower_bound

    (0..distance).each do |dx|
      (0..distance).each do |dy|
        explore(Coordinate.new(curr_upper_bound.x + dx, curr_upper_bound.y + dy))
        explore(Coordinate.new(curr_lower_bound.x + dx, curr_lower_bound.y + dy))
      end
    end
  end

  def explore(coord)
    if @grid.painted?(coord)
      return @grid.value(coord)
    end

    log.debug "explore #{coord}"
    val = check_coordinate(@program, coord)
    is_beam = val == 1
    @grid.paint!(coord, is_beam)

    if is_beam
      if (coord.x > @upper_bound.x || (coord.x == @upper_bound.x && coord.y < @upper_bound.y))
        @upper_bound = coord
        @upper_bounds << coord
      end

      if (coord.y > @lower_bound.y || (coord.y == @lower_bound.y && coord.x < @lower_bound.x))
        @lower_bound = coord
        @lower_bounds << coord
      end
    end

    is_beam
  end

  def find_place_for_ship
    diff = (@ship_size - 1)

    @upper_bounds.each do |upper_bound|
      # given top right corner of box (upper bound), check if bottom right corner
      # of box is in beam. if so, return top left corner of box.
      if explore(Coordinate.new(upper_bound.x - diff, upper_bound.y + diff))
        return Coordinate.new(upper_bound.x - diff, upper_bound.y)
      end
    end

    nil
  end

  def to_s
    @grid.to_s {|_, v| v.nil? ? ' ' : (v ? '#' : '.')}
  end
end

