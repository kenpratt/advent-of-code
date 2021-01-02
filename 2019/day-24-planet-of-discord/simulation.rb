DIRECTIONS = [:left, :right, :up, :down]

Coord = Struct.new(:level, :x, :y)

class Simulation
  def initialize(map_str, recursive)
    rows = map_str.split("\n").map {|line| line.each_char.map {|c| self.class.parse_char(c)}}

    @width = rows.first.size
    @height = rows.size
    @recursive = recursive
    @neighbour_cache = {}

    @center_x = @width / 2
    @center_y = @height / 2

    @alive = Set.new
    rows.each_with_index do |row, y|
      row.each_with_index do |val, x|
        @alive << Coord.new(0, x, y) if val
      end
    end
  end

  def run_until_repeat
    history = Set.new

    while !history.include?(@alive)
      history << @alive.clone
      tick
    end

    self
  end

  def run(cycles)
    cycles.times do
      tick
    end

    self
  end

  def tick
    new_alive = []
    visit_locations do |cell|
      new_alive << cell if new_value(cell)
    end
    @alive = Set.new(new_alive)
  end

  def visit_locations
    if !@recursive
      # visit the whole grid
      (0...@height).map do |y|
        (0...@width).map do |x|
          yield Coord.new(0, x, y)
        end
      end
    else
      # visit all knows levels, plus one lower and higher in case of bug
      # infection to new recursion level
      min_level, max_level = level_bounds
      ((min_level - 1)..(max_level + 1)).map do |level|
        (0...@height).map do |y|
          (0...@width).map do |x|
            yield Coord.new(level, x, y) unless x == @center_x && y == @center_y
          end
        end
      end
    end
  end

  def neighbours(cell)
    @neighbour_cache[cell] ||= calculate_neighbours(cell)
  end

  def calculate_neighbours(cell)
    DIRECTIONS.map {|d| Set.new(calculate_neighbours_in_direction(cell, d))}.inject(:|)
  end

  def calculate_neighbours_in_direction(cell, direction)
    nx = cell.x + case direction
    when :left then -1
    when :right then 1
    else 0
    end

    ny = cell.y + case direction
    when :up then -1
    when :down then 1
    else 0
    end

    beyond_edge = nx < 0 || nx == @width || ny < 0 || ny == @height

    if !@recursive
      if beyond_edge
        # hit the edge
        []
      else
        # neighbour
        [Coord.new(cell.level, nx, ny)]
      end
    else
      if nx == @center_x && ny == @center_y
        # recurse into inner board, treating whole edge as adjacent
        lvl = cell.level + 1
        case direction
        when :left
          # right edge
          (0...@height).map {|y| Coord.new(lvl, @width - 1, y)}
        when :right
          # left edge
          (0...@height).map {|y| Coord.new(lvl, 0, y)}
        when :up
          # bottom edge
          (0...@width).map {|x| Coord.new(lvl, x, @height - 1)}
        when :down
          # top edge
          (0...@width).map {|x| Coord.new(lvl, x, 0)}
        end
      elsif beyond_edge
        # recurse into outer board, in same direction from center
        calculate_neighbours_in_direction(Coord.new(cell.level - 1, @center_x, @center_y), direction)
      else
        # normal neighbour
        [Coord.new(cell.level, nx, ny)]
      end
    end
  end

  def new_value(cell)
    adjacent = neighbours(cell)
    num_adjacent = adjacent.count {|n| @alive.include?(n)}

    if @alive.include?(cell)
      # A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
      num_adjacent == 1
    else
      # An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
      num_adjacent == 1 || num_adjacent == 2
    end
  end

  def self.parse_char(char)
    case char
    when '#'
      true
    when '.'
      false
    else
      raise "Unknown char: #{c.inspect}"
    end
  end

  def level_bounds
    min_level = 0
    max_level = 0
    @alive.each do |cell|
      level = cell.level
      min_level = level if level < min_level
      max_level = level if level > max_level
    end
    [min_level, max_level]
  end

  def to_s
    min_level, max_level = level_bounds
    (min_level..max_level).map do |level|
      (@recursive ? "Depth #{level}:\n" : "") +
      (0...@height).map do |y|
        (0...@width).map do |x|
          if @recursive && x == @center_x && y == @center_y
            '?'
          else
            @alive.include?(Coord.new(level, x, y)) ? '#' : '.'
          end
        end.join('')
      end.join("\n")
    end.join("\n\n")
  end

  def biodiversity_rating
    output = 0
    f = 1
    (0...@height).map do |y|
      (0...@width).map do |x|
        output += f if @alive.include?(Coord.new(0, x, y))
        f *= 2
      end
    end
    output
  end

  def count_bugs
    @alive.size
  end
end
