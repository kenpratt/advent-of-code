require_relative 'grid'

class Simulation
  def initialize(map_str)
    rows = map_str.split("\n").map {|line| line.each_char.map {|c| self.class.parse_char(c)}}
    @map = StaticGrid.from_rows(rows)
    @neighbours = @map.coords.map {|c| [c, c.neighbours.values]}.to_h
  end

  def run_until_repeat
    history = Set.new
    history << self.to_s

    found_duplicate = false
    while !found_duplicate
      tick

      s = self.to_s
      if history.include?(s)
        found_duplicate = true
      else
        history << s
      end
    end

    self
  end

  def tick
    new_cells = @map.coords.map do |c|
      [c, new_value(c)]
    end.to_h
    @map.set_cells!(new_cells)
  end

  def new_value(c)
    alive = @map.value(c)
    num_adjacent = @neighbours[c].count {|n| @map.value(n)}
    if alive
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

  def to_s
    @map.to_s(borders: false) {|_, alive| alive ? '#' : '.'}
  end

  def biodiversity_rating
    output = 0
    x = 1
    @map.cells.each do |_, v|
      output += x if v
      x *= 2
    end
    output
  end
end
