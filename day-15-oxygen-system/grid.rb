Bounds = Struct.new(:left, :right, :top, :bottom) do
  include Enumerable
  
  def &(other)
    Bounds.new(
      [self.left, other.left].min,
      [self.right, other.right].max,
      [self.top, other.top].min,
      [self.bottom, other.bottom].max,
    )
  end

  def width
    self.right - self.left + 1
  end

  def height
    self.bottom - self.top + 1
  end

  def expand!(point)
    self.left = point.x if point.x < self.left
    self.right = point.x if point.x > self.right
    self.top = point.y if point.y < self.top
    self.bottom = point.y if point.y > self.bottom
    self
  end

  def render_grid(borders:, &proc)
    rows = rendered_cells(&proc)
    if borders
      top_bottom = '+' + ('-' * rows[0].size) + '+' + "\n"
      top_bottom + rows.map {|cells| '|' + cells.join('') + '|'}.join("\n") + "\n" + top_bottom
    else
      rows.map {|cells| cells.join('')}.join("\n") + "\n"
    end
  end

  def rendered_cells(&proc)
    (top..bottom).map do |y|
      (left..right).map do |x|
        proc.call(Coordinate.new(x, y))
      end
    end
  end

  def each_cell(&proc)
    (top..bottom).each do |y|
      (left..right).each do |x|
        proc.call(Coordinate.new(x, y))
      end
    end
  end
end

class GrowableGrid
  attr_reader :cells

  def initialize
    @cells = {}
  end

  def value(coord)
    @cells[coord]
  end

  def paint!(coord, value)
    log.debug "paint #{coord} #{value}"
    @cells[coord.clone] = value
  end

  def fully_painted?
    @cells.size == bounds.width * bounds.height
  end

  def bounds
    x_values = @cells.keys.map {|c| c.x}
    y_values = @cells.keys.map {|c| c.y}
    Bounds.new(
      x_values.min,
      x_values.max,
      y_values.min,
      y_values.max,
    )
  end

  def top_left_corner
    b = bounds
    Coordinate.new(b.left, b.top)
  end

  def cells_with_value(target) 
    res = []
    bounds.each_cell do |c| 
      res << c if value(c) == target
    end
    res
  end  

  def to_s(&proc)
    bounds.render_grid(borders: true) do |c|
      proc.call(c, @cells[c])
    end
  end
end

Coordinate = Struct.new(:x, :y) do
  DIRECTIONS = [:left, :right, :up, :down]

  def move(direction, amount)
    case direction
    when :left
      Coordinate.new(self.x - amount, y)
    when :right
      Coordinate.new(self.x + amount, y)
    when :up
      Coordinate.new(self.x, self.y - amount)
    when :down
      Coordinate.new(self.x, self.y + amount)
    else
      raise "Unknown direction: #{direction}"
    end
  end
  def move_left(amount); move(:left, amount); end
  def move_right(amount); move(:right, amount); end
  def move_up(amount); move(:up, amount); end
  def move_down(amount); move(:down, amount); end

  def move!(direction, amount)
    case direction
    when :left
      self.x -= amount
    when :right
      self.x += amount
    when :up
      self.y -= amount
    when :down
      self.y += amount
    else
      raise "Unknown direction: #{direction}"
    end
  end  
  def move_left!(amount); move!(:left, amount); end
  def move_right!(amount); move!(:right, amount); end
  def move_up!(amount); move!(:up, amount); end
  def move_down!(amount); move!(:down, amount); end

  def neighbours
    DIRECTIONS.map do |direction|
      [direction, move(direction, 1)]
    end.to_h
  end

  def manhattan_distance(other)
    (other.x - self.x).abs + (other.y - self.y).abs
  end

  def path_to(other, &is_coord_visitable)
    AStar.find_path(self, other, &is_coord_visitable)
  end
end

class AStar
  def self.find_path(from_coord, to_coord, &is_coord_visitable)
    astar = self.new(from_coord, to_coord, &is_coord_visitable)
    astar.run
  end

  def initialize(from_coord, to_coord, &is_coord_visitable)
    @from_coord = from_coord
    @to_coord = to_coord
    @is_coord_visitable = is_coord_visitable
    @open_set = Set.new()
    @came_from = {}
    @direction_to = {}
    @f_score = Hash.new(1_000_000) # guessed distance from node to end
    @g_score = Hash.new(1_000_000) # known distance from start to node
  end

  def run
    @open_set << @from_coord
    @f_score[@from_coord] = heuristic(@from_coord)
    @g_score[@from_coord] = 0

    while !@open_set.empty?
      current = @open_set.min_by {|c| @f_score[c]}

      if current == @to_coord
        return reconstruct_path(current)
      end
      
      @open_set.delete(current)
      visitable_neighbours(current).each do |direction, neighbour|
        tentative_g_score = @g_score[current] + 1
        if tentative_g_score < @g_score[neighbour]
          # This path to neighbor is better than any previous one. Record it!
          @came_from[neighbour] = current
          @direction_to[neighbour] = direction
          @g_score[neighbour] = tentative_g_score
          @f_score[neighbour] = tentative_g_score + heuristic(neighbour)
          @open_set << neighbour
        end
      end
    end

    nil
  end

  def heuristic(coord)
    coord.manhattan_distance(@to_coord)
  end

  def visitable_neighbours(coord)
    coord.neighbours.select {|dir, c| @is_coord_visitable.call(c)}
  end

  def reconstruct_path(coord)
    path = []
    while coord
      direction = @direction_to[coord]
      path << direction if direction
      coord = @came_from[coord]
    end
    return path.reverse
  end
end