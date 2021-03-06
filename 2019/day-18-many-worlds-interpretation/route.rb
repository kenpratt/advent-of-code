Route = Struct.new(:entrance, :from, :to, :distance, :necessary_keys, :keys_along_route) do
  def flip
    self.class.new(entrance, to, from, distance, necessary_keys, keys_along_route)
  end

  def have_necessary_keys?(have_keys)
    necessary_keys.subset?(have_keys)
  end

  def have_all_keys_along_route?(have_keys)
    keys_along_route.subset?(have_keys)
  end

  def <=>(other)
    self.distance <=> other.distance
  end

  def to_array
    [
      entrance,
      from,
      to,
      distance,
      necessary_keys.to_a,
      keys_along_route.map(&:value).to_a,
    ]
  end
  
  def self.from_array(arr)
    entrance, from, to, distance, necessary_keys, keys_along_route = *arr
    self.new(entrance, from, to, distance, Set.new(necessary_keys), Set.new(keys_along_route))
  end
end

class Routes
  attr_reader :routes_set, :lookup_map, :to_visit #, :entrance_for_destination

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
    @lookup_map = Hash.new {|h, k| h[k] = Hash.new {|h2, k2| h2[k2] = {}}}
    #@entrance_for_destination = {}
    @to_visit = Set.new

    routes.each {|route| add(route)}
  end

  def get(entrance, from, to)
    @lookup_map[entrance][from][to]
  end
  
  def add(route)
    @routes_set << route
    @lookup_map[route.entrance][route.from][route.to] = route
    #@entrance_for_destination[route.to] = route.entrance
    @to_visit << route.to
  end

  def entrances
    @lookup_map.keys.sort
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