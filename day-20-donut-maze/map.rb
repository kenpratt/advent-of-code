require_relative 'grid'
require_relative 'route'

class Tile
  attr_accessor :location

  def visitable?
    raise "subclass must implement"
  end
end

class Passage < Tile
  attr_accessor :label

  def initialize
    @label = nil
  end

  def visitable?
    true
  end

  def entrance?
    @label == 'AA'
  end

  def exit?
    @label == 'ZZ'
  end
end

class Wall < Tile
  def visitable?
    false
  end
end

class Empty < Tile
  def visitable?
    false
  end
end

class LabelPart < Tile
  attr_reader :value

  def initialize(value)
    @value = value
  end

  def visitable?
    false
  end
end

class Map
  attr_reader :grid

  POSSIBLE_LABEL_PARTS = Set.new(('A'..'Z').to_a)
  WALL = '#'
  PASSAGE = '.'
  EMPTY = ' '

  def initialize(map_str)
    rows_with_chars = map_str.split("\n").map {|s| s.split('')}

    rows = rows_with_chars.map do |row|
      row.map do |s|
        Map.parse_tile(s)
      end
    end

    @grid = StaticGrid.from_rows(rows)
    @grid.cells.each do |location, tile|
      tile.location = location
    end

    # apply labels
    passages.each {|p| try_to_label_passage!(p)}
  end

  def self.load(filename)
    map_str = File.read(filename)
    self.new(map_str)
  end

  def self.parse_tile(s)
    if s == WALL
      Wall.new
    elsif s == EMPTY
      Empty.new
    elsif s == PASSAGE
      Passage.new
    elsif POSSIBLE_LABEL_PARTS.include?(s)
      LabelPart.new(s)
    else
      raise "Unknown tile: #{s}"
    end
  end

  def passages
    Set.new(@grid.cells.values.select {|tile| tile.is_a?(Passage)})
  end

  def labelled_passages
    passages.select {|p| p.label}
  end

  def try_to_label_passage!(passage)
    passage.location.neighbours.each do |direction, location|
      first_tile = @grid.value(location)
      if first_tile.is_a?(LabelPart)
        second_tile = @grid.value(location.move(direction))
        raise "Expected a label" unless second_tile.is_a?(LabelPart)

        chars = [first_tile.value, second_tile.value]
        chars = chars.reverse if direction == :left || direction == :up
        passage.label = chars.join('')
      end
    end
  end

  def location_visitable?(location)
    @grid.cells[location].visitable?
  end

  def build_routes(filename)
    routes = Routes.new(filename, [])

    i = 1
    labelled_passages.each do |from_tile|
      labelled_passages.each do |to_tile|
        if from_tile != to_tile && !to_tile.entrance? && !from_tile.exit?
          log.debug "caching route #{i}"
          add_route(routes, from_tile, to_tile)
          i += 1
        end
      end
    end

    routes
  end

  def tile_value_for_route(tile)
    [tile.location.x, tile.location.y, tile.label]
  end

  def add_route(routes, from_tile, to_tile)
    from_val = tile_value_for_route(from_tile)
    to_val = tile_value_for_route(to_tile)

    if (reverse_route = routes.get(to_val, from_val))
      # shortcut
      route = reverse_route.flip
      routes.add(route)
      return  
    end

    if from_tile.label == to_tile.label
      # special case -- warp
      route = Route.new(from_val, to_val, 1)
      routes.add(route)
      return
    end

    path = from_tile.location.path_to(to_tile.location) {|l| location_visitable?(l)}
    if path
      route = Route.new(from_val, to_val, path.size)
      routes.add(route)
    end
  end
end
