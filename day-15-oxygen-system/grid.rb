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
        yield Coordinate.new(x, y)
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
  
  def num_painted(&proc)
    @cells.size
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

  def to_s(&proc)
    bounds.render_grid(borders: true) do |c|
      proc.call(c, @cells[c])
    end
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
