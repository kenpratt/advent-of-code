Route = Struct.new(:from, :to, :distance, :necessary_keys, :keys_along_route) do
  def flip
    self.class.new(to, from, distance, necessary_keys, keys_along_route)
  end

  def <=>(other)
    self.distance <=> other.distance
  end

  def to_array
    [
      from,
      to,
      distance,
      necessary_keys.to_a,
      keys_along_route.map(&:value).to_a,
    ]
  end
  
  def self.from_array(arr)
    from, to, distance, necessary_keys, keys_along_route = *arr
    self.new(from, to, distance, Set.new(necessary_keys), Set.new(keys_along_route))
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

  def lookup(from, to)
    @lookup_map[from][to]
  end

  def add(route)
    @routes_set << route
    @lookup_map[route.from][route.to] = route
  end
end