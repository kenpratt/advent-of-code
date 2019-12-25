require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :map, :location, :to_visit,
    :steps, :collected_keys, :collected_key_order, :visited,
    :f_score, :g_score, :h_score

  def initialize(map, location, to_visit)
    @map = map
    @location = location
    @to_visit = to_visit

    @steps = 0
    @collected_keys = Set.new()
    @collected_key_order = []
    @visited = Set.new()

    update_scores
  end

  def complete?
    @to_visit.empty?
  end

  def update_scores
    # f(n) = g(n) + h(n)
    # g(n) = known distance from start to node
    # h(n) = guessed distance from node to end (must be <= actual distance)
    @g_score = calculate_g_score
    @h_score = calculate_h_score
    @f_score = @g_score + @h_score
  end

  def calculate_g_score
    @steps
  end

  def calculate_h_score
    @to_visit.map do |tile|
      route = @location.path_to(tile.location) {|l| location_visitable?(l)}
      route ? route.size : 1_000_000
    end.max
  end

  def next_paths
    # calculate possible path to each unvisited item (door/key) that is reachable
    # and return new paths including the distance walked to them
    out = []

    @to_visit.each do |tile|
      route = @location.path_to(tile.location) {|l| location_visitable?(l)}
      
      # if route is nil, it's blocked by a door, so filter out
      if route
        # TODO consruct a new Path at the tile location:
        # - adding the steps to get there
        # - marking the new location as visited (removing from to_visit)
        # - and recalculating scores
        # - (need a clone func)        
        binding.pry
        new_path = TODO
        out << new_path
      end
    end

    out
  end
  
  def location_visitable?(location)
    @map.location_visitable?(location, @collected_keys)
  end

  def <=>(other)
    self.f_score <=> other.f_score
  end
end

class Solver
  def self.run(map)
    astar = self.new(map)
    astar.run
  end

  def initialize(map)
    @map = map
    starting_path = Path.new(map, map.starting_location, map.tiles_to_visit)
    @open_set = SortedSet.new([starting_path])
  end

  def run
    while !@open_set.empty?
      current_path = @open_set.first

      if current_path.complete?
        return current_path
      end
      
      @open_set.delete(current_path)
      # TODO(maybe) re-incorporate something from algorithm about only tracking one item per "location"
      # (where here "location" is maybe the set of seen items, or set of seen items + location?)
      @open_set += current_path.next_paths
    end

    nil
  end

  def heuristic(coord)
    coord.manhattan_distance(@to_coord)
  end

  def visitable_neighbours(coord)
    coord.neighbours.select {|dir, c| @is_coord_visitable.call(c)}
  end
 end