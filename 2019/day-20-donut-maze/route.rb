Route = Struct.new(:from, :to, :distance) do
  def flip
    self.class.new(to, from, distance)
  end

  def <=>(other)
    self.distance <=> other.distance
  end

  def to_array
    [
      from,
      to,
      distance,
    ]
  end
  
  def self.from_array(arr)
    from, to, distance = *arr
    self.new(from, to, distance)
  end
end

class Routes
  attr_reader :routes_set, :lookup_map

  def self.load(filename)
    routes_str = File.read(filename)
    routes_arr = JSON.parse(routes_str)
    routes = routes_arr.map {|arr| Route.from_array(arr)}
    self.new(filename, routes)
  end

  def dump
    str = JSON.generate(@routes_set.map(&:to_array))
    File.open(@filename, 'w') {|f| f << str}
  end

  def initialize(filename, routes)
    @filename = filename
    @routes_set = Set.new
    @lookup_map = Hash.new {|h, k| h[k] = {}}

    routes.each {|route| add(route)}
  end

  def entrance
    @lookup_map.keys.find {|location| location.last == 'AA'}
  end

  def get(from, to)
    @lookup_map[from][to]
  end
  
  def all_from(from)
    @lookup_map[from].values
  end
  
  def add(route)
    @routes_set << route
    @lookup_map[route.from][route.to] = route
  end

  def minimum_spanning_tree_distance(keys_to_include)
    starting_key = keys_to_include[0]
    seen = Set.new([starting_key])
    left = Set.new(keys_to_include[1..-1])
    
    distance = 0
    cheapest_route = {}

    left.map do |next_key|
      cheapest_route[next_key] = get(starting_key, next_key)
    end

    while left.any?
      next_key = left.min_by {|k| cheapest_route[k].distance}
      route = cheapest_route[next_key]

      distance += route.distance
      seen << next_key
      left.delete(next_key)

      @lookup_map[next_key].map do |to_key, maybe_route|
        if left.include?(to_key) && maybe_route.distance < cheapest_route[to_key].distance
          cheapest_route[to_key] = maybe_route
        end
      end
    end

    distance
  end
end