require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :map, :location, :to_visit,
    :visited, :collected_keys, :steps,
    :f_score, :g_score, :h_score

  def self.initial(map, location, to_visit)
    self.new(map, location, to_visit, Set.new, Set.new, 0)
  end

  def initialize(map, location, to_visit, visited, collected_keys, steps)
    @map = map
    @location = location
    @to_visit = to_visit

    @visited = visited
    @collected_keys = collected_keys
    @steps = steps

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
    @to_visit.size
  end

  def calculate_h_score_old
    return 0 if @to_visit.empty?

    @to_visit.map do |tile|
      route = @location.path_to(tile.location) {|l| location_visitable?(l, pretend_all_keys_collected: true)}
      raise "Couldn't find route to tile - unexpected" if route.nil?
      route.size
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
        new_path = move_to_tile(tile, route)
        out << new_path
      end
    end

    out
  end

  def move_to_tile(tile, route)
    next_location = tile.location
    next_steps = @steps + route.size

    next_to_visit = @to_visit - [tile]
    next_visited = @visited + [tile.value]

    if tile.is_a?(Key)
      next_collected_keys = @collected_keys + [tile.value]
    else
      next_collected_keys = @collected_keys
    end

    self.class.new(@map, next_location, next_to_visit, next_visited, next_collected_keys, next_steps)
  end
  
  def location_visitable?(location, pretend_all_keys_collected: false)
    @map.location_visitable?(location, @collected_keys, pretend_all_keys_collected)
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
    starting_path = Path.initial(map, map.starting_location, map.tiles_to_visit)
    @open_set = SortedSet.new([starting_path])
    @best_for_location_and_visited = Hash.new {|h, key| h[key] = {}}
  end

  def run
    i = 1

    while !@open_set.empty?
      current_path = @open_set.first

      log.debug "Iteration #{i}, #{@open_set.size} paths, first one size #{current_path.steps} with #{current_path.visited.size} items visited" if i % 100 == 0

      binding.pry if i == 200

      if current_path.complete?
        return current_path
      end
      
      @open_set.delete(current_path)

      # TODO(maybe) re-incorporate something from algorithm about only tracking one item per "location"
      # (where here "location" is maybe the set of seen items, or set of seen items + location?)

      current_path.next_paths.each do |next_path|
        current_best = @best_for_location_and_visited[next_path.location][next_path.visited]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_location_and_visited[next_path.location][next_path.visited] = next_path
        end
      end

      i += 1
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