require 'logger'

$log = Logger.new(STDOUT)
#$log.level = Logger::DEBUG
$log.level = Logger::INFO
def log; $log; end

INPUT_FILE = File.join(__dir__, 'input.txt')

def part1(input)
  wire1, wire2 = *input
  log.debug wire1.inspect
  log.debug wire2.inspect  
  
  bounds1 = find_bounds(wire1)
  log.debug bounds1.inspect
  log.debug bounds1.width
  log.debug bounds1.height

  bounds2 = find_bounds(wire2)
  log.debug bounds2.inspect
  log.debug bounds2.width
  log.debug bounds2.height

  bounds = bounds1 & bounds2
  log.debug bounds.inspect
  log.debug bounds.width
  log.debug bounds.height

  grid = Grid.new(bounds)
  
  origin = Coordinate.new(0, 0)
  walk_wire(wire1, origin) do |point, _|
    grid.set!(point, true)
  end

  intersections = []
  walk_wire(wire2, origin) do |point, _|
    intersections << point if grid.get(point)
  end
  log.debug intersections.inspect

  best = intersections.min_by {|c| origin.manhattan_distance(c)}
  log.debug best.inspect

  origin.manhattan_distance(best)
end

def part2(input)
  wire1, wire2 = *input
  log.debug wire1.inspect
  log.debug wire2.inspect  
  
  bounds1 = find_bounds(wire1)
  log.debug bounds1.inspect
  log.debug bounds1.width
  log.debug bounds1.height

  bounds2 = find_bounds(wire2)
  log.debug bounds2.inspect
  log.debug bounds2.width
  log.debug bounds2.height

  bounds = bounds1 & bounds2
  log.debug bounds.inspect
  log.debug bounds.width
  log.debug bounds.height

  grid = Grid.new(bounds)
  
  origin = Coordinate.new(0, 0)
  walk_wire(wire1, origin) do |point, walked1|
    grid.set!(point, walked1)
  end

  intersections = []
  walk_wire(wire2, origin) do |point, walked2|
    if (walked1 = grid.get(point))
      intersections << [point, walked1 + walked2]
    end
  end
  log.debug intersections.inspect

  best = intersections.min_by {|point, walked| walked}
  log.debug best.inspect

  best[1]
end

def process_input(input)
  lines = input.split("\n")
  wire1 = lines[0].split(',').map {|s| parse_instruction(s)}
  wire2 = lines[1].split(',').map {|s| parse_instruction(s)}
  [wire1, wire2]
end

def walk_wire(wire, point)
  distance_walked = 0
  wire.each do |instruction|
    res = case instruction.op
    when 'L'
      instruction.amount.times do
        point = point.move_left(1)
        distance_walked += 1
        yield point, distance_walked
      end
    when 'R'
      instruction.amount.times do
        point = point.move_right(1)
        distance_walked += 1
        yield point, distance_walked
      end
    when 'U'
      instruction.amount.times do
        point = point.move_up(1)
        distance_walked += 1
        yield point, distance_walked
      end
    when 'D'
      instruction.amount.times do
        point = point.move_down(1)
        distance_walked += 1
        yield point, distance_walked
      end
    else
      raise 'Unknown instruction'
    end
  end  
end

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
  def move_left(amount)
    Coordinate.new(self.x - amount, y)
  end

  def move_right(amount)
    Coordinate.new(self.x + amount, y)
  end

  def move_up(amount)
    Coordinate.new(self.x, self.y - amount)
  end
  
  def move_down(amount)
    Coordinate.new(self.x, self.y + amount)
  end

  def manhattan_distance(other)
    (other.x - self.x).abs + (other.y - self.y).abs
  end
end

Instruction = Struct.new(:op, :amount)

def parse_instruction(s)
  Instruction.new(s[0], s[1..-1].to_i)
end

def find_bounds(wire)
  bounds = Bounds.new(0, 0, 0, 0)
  point = Coordinate.new(0, 0)
  wire.each do |instruction|
    res = case instruction.op
    when 'L'
      point = point.move_left(instruction.amount)
      bounds.left = point.x if point.x < bounds.left
    when 'R'
      point = point.move_right(instruction.amount)
      bounds.right = point.x if point.x > bounds.right
    when 'U'
      point = point.move_up(instruction.amount)
      bounds.top = point.y if point.y < bounds.top
    when 'D'
      point = point.move_down(instruction.amount)
      bounds.bottom = point.y if point.y > bounds.bottom
    else
      raise 'Unknown instruction'
    end
  end
  bounds
end
  
def main
  if ARGV[0] == 'debug'
    log.level = Logger::DEBUG
  end

  lines = File.read(INPUT_FILE)
  input = process_input(lines)

  log.info "Part 1:"
  log.info part1(input)

  log.info "Part 2:"
  log.info part2(input)
end

if __FILE__ == $0
  main
end