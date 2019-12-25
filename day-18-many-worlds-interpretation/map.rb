require_relative 'grid'

class Tile
  attr_accessor :location

  def visitable?
    raise "subclass must implement"
  end
end

class Corridor < Tile
  def visitable?(have_keys, pretend_all_keys_collected)
    true
  end
end

class Wall < Tile
  def visitable?(have_keys, pretend_all_keys_collected)
    false
  end
end

class Entrance < Tile
  def visitable?(have_keys, pretend_all_keys_collected)
    true
  end
end

class Key < Tile
  attr_reader :value

  def initialize(value)
    @value = value
  end

  def visitable?(have_keys, pretend_all_keys_collected)
    true
  end
end

class Door < Tile
  attr_reader :value

  def initialize(value)
    @value = value
    @key_value = value.downcase
  end

  def visitable?(have_keys, pretend_all_keys_collected)
    pretend_all_keys_collected || have_keys.include?(@key_value)
  end
end

class Map
  attr_reader :grid, :entrance, :keys, :doors

  POSSIBLE_KEYS = Set.new(('a'..'z').to_a)
  POSSIBLE_DOORS = Set.new(('A'..'Z').to_a)
  WALL = '#'
  CORRIDOR = '.'
  ENTRANCE = '@'

  def initialize(map_str)
    rows_with_chars = map_str.split("\n").map {|s| s.split('')}

    rows = rows_with_chars.map do |row|
      row.map {|s| Map.parse_tile(s)}
    end

    @grid = StaticGrid.from_rows(rows)
    @grid.cells.each do |location, tile|
      tile.location = location
    end

    @entrance = grid.cells.values.find {|tile| tile.is_a?(Entrance)}
    @keys = Set.new(grid.cells.values.select {|tile| tile.is_a?(Key)})
    @doors = Set.new(grid.cells.values.select {|tile| tile.is_a?(Door)})
  end

  def self.parse_tile(s)
    if s == WALL
      Wall.new
    elsif s == CORRIDOR
      Corridor.new
    elsif s == ENTRANCE
      Entrance.new
    elsif POSSIBLE_KEYS.include?(s)
      Key.new(s)
    elsif POSSIBLE_DOORS.include?(s)
      Door.new(s)
    else
      raise "Unknown tile: #{s}"
    end
  end

  def starting_location
    @entrance.location
  end

  def tiles_to_visit
    @keys + @doors
  end

  def location_visitable?(location, have_keys, pretend_all_keys_collected)
    @grid.cells[location].visitable?(have_keys, pretend_all_keys_collected)
  end
end
