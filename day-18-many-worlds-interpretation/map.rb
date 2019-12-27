require_relative 'grid'
require_relative 'route'

class Tile
  attr_accessor :location

  def visitable_assuming_all_keys?
    raise "subclass must implement"
  end

  def visitable_with_keys?(keys)
    raise "subclass must implement"
  end
end

class Corridor < Tile
  def visitable_assuming_all_keys?
    true
  end

  def visitable_with_keys?(keys)
    true
  end
end

class Wall < Tile
  def visitable_assuming_all_keys?
    false
  end

  def visitable_with_keys?(keys)
    false
  end
end

class Entrance < Tile
  def visitable_assuming_all_keys?
    true
  end

  def visitable_with_keys?(keys)
    true
  end
end

class Key < Tile
  attr_reader :value

  def initialize(value)
    @value = value
  end

  def visitable_assuming_all_keys?
    true
  end

  def visitable_with_keys?(keys)
    true
  end
end

class Door < Tile
  attr_reader :value, :key_value

  def initialize(value)
    @value = value
    @key_value = value.downcase
  end

  def visitable_assuming_all_keys?
    true
  end

  def visitable_with_keys?(keys)
    keys.include?(@key_value)
  end
end

class Map
  attr_reader :grid

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
  end

  def self.load(filename)
    map_str = File.read(filename)
    self.new(map_str)
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
    entrance.location
  end

  def entrance
    grid.cells.values.find {|tile| tile.is_a?(Entrance)}
  end

  def keys
    Set.new(grid.cells.values.select {|tile| tile.is_a?(Key)})
  end

  def location_visitable_with_keys?(location, keys)
    @grid.cells[location].visitable_with_keys?(keys)
  end

  def location_visitable_assuming_all_keys?(location)
    @grid.cells[location].visitable_assuming_all_keys?
  end

  def build_routes(filename)
    routes = Routes.new(filename, [])

    i = 1
    key_locations = keys.map(&:location)
    log.debug "caching routes (#{key_locations.size} key locations) = #{(key_locations.size + 1) * (key_locations.size - 1)} expected routes"
    ([entrance.location] + key_locations).each do |from_tile|
      key_locations.each do |to_tile|
        if from_tile != to_tile
          log.debug "caching route #{i}"
          add_route(routes, from_tile, to_tile)
          i += 1
        end
      end
    end

    routes
  end

  def add_route(routes, from_location, to_location)
    from_tile = @grid.cells[from_location]
    to_tile = @grid.cells[to_location]

    from_val = case from_tile
    when Entrance
      'E'
    when Key
      from_tile.value
    else
      raise 'Unknown from_val'
    end

    to_val = case to_tile
    when Key
      to_tile.value
    else
      raise 'Unknown to_val'
    end

    if (reverse_route = routes.lookup(from_val, to_val))
      # shortcut
      route = reverse_route.flip
      routes.add(route)
      return  
    end

    raw_route = from_location.path_to(to_location) {|l| location_visitable_assuming_all_keys?(l)}
    raise "Couldn't find route between locations - unexpected" if raw_route.nil?

    distance = raw_route.size

    # ignore last one as it's the destination
    tiles_along_route = raw_route[0...-1].map {|_direction, location| @grid.cells[location]}

    doors_along_route = tiles_along_route.select {|tile| tile.is_a?(Door)}
    keys_along_route = tiles_along_route.select {|tile| tile.is_a?(Key)}

    necessary_keys = doors_along_route.map(&:key_value)
    
    route = Route.new(from_val, to_val, distance, Set.new(necessary_keys), Set.new(keys_along_route))
    routes.add(route)

    # check for routes with not all required keys (in case of cycles etc)
    # if necessary_keys.size > 1
    #   to_check = [[]] + (1..(necessary_keys.size - 1)).flat_map {|n| necessary_keys.combination(n).to_a}
    #   to_check.each do |with_keys|
    #     route_with_keys = from_location.path_to(to_location) {|l| location_visitable_with_keys?(l, with_keys)}
    #     if route_with_keys
    #       raise "Did not expect to find route with subset of keys #{from_location} #{to_location} #{with_keys}"
    #     end
    #   end
    # end
  end
end
