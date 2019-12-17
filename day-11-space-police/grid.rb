Bounds = Struct.new(:left, :right, :top, :bottom) do
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

  def render_grid
    (top..bottom).map do |y|
      (left..right).map do |x|
        yield Coordinate.new(x, y)
      end.join('')
    end.join("\n")
  end
end

class Grid
  def initialize(bounds)
    @bounds = bounds
    @width = @bounds.width
    @height = @bounds.height
    @left = @bounds.left
    @top = @bounds.top
    @cells = Array.new(@width * @height)
  end

  def cell_index(coord)
    x = coord.x - @left
    y = coord.y - @top
    raise "x coordinate out of bounds: #{coord.inspect} #{@bounds.inspect}" if x < 0 || x >= @width
    raise "y coordinate out of bounds: #{coord.inspect} #{@bounds.inspect}" if y < 0 || y >= @height
    x + (y * @width)
  end

  def get(coord)
    i = cell_index(coord)
    @cells[i]
  end

  def set!(coord, value)
    i = cell_index(coord)
    @cells[i] = value
  end
end

Coordinate = Struct.new(:x, :y) do
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

  def manhattan_distance(other)
    (other.x - self.x).abs + (other.y - self.y).abs
  end
end
