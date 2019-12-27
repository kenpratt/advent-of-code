require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :map, :location, :to_visit, :collected_keys, :steps, :f_score, :g_score, :h_score

  def self.initial(map, location, to_visit)
    self.new(map, location, to_visit, Set.new, 0)
  end

  def initialize(map, location, to_visit, collected_keys, steps)
    @map = map
    @location = location
    @to_visit = to_visit

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
    return 0

    #return 0 if @to_visit.empty?

    #raise "TODO"

    # return @to_visit.size

    # if @to_visit.size == 1
    #   tile = @to_visit.first
    #   return @map.distance_assuming_all_keys(@location, tile.location)
    # end

    # furthest_pair = nil
    # distance_between_furthest_pair = 0

    # @to_visit.each do |from_tile|
    #   @to_visit.each do |to_tile|
    #     next if from_tile == to_tile
    #     distance = @map.distance_assuming_all_keys(from_tile.location, to_tile.location)
    #     binding.pry if distance.nil?
    #     if distance > distance_between_furthest_pair
    #       distance_between_furthest_pair = distance
    #       furthest_pair = [from_tile, to_tile]
    #     end
    #   end
    # end

    # distance_to_closer_one = furthest_pair.map {|tile| @map.distance_assuming_all_keys(@location, tile.location)}.min

    # distance_to_closer_one + distance_between_furthest_pair
  end

  def next_paths
    # calculate possible path to each unvisited item (door/key) that is reachable
    # and return new paths including the distance walked to them
    out = []

    @to_visit.each do |tile|
      route = @location.path_to(tile.location) {|l| @map.location_visitable_with_keys?(l, @collected_keys)}

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

    if tile.is_a?(Key)
      next_collected_keys = @collected_keys + [tile.value]
    else
      next_collected_keys = @collected_keys
    end

    self.class.new(@map, next_location, next_to_visit, next_collected_keys, next_steps)
  end

  def <=>(other)
    self.f_score <=> other.f_score
  end
end

class Solver
  def self.run(map_file)
    routes_file = File.join(
      File.dirname(map_file),
      'routes-' + File.basename(map_file),
    )

    if !File.exist?(routes_file)
      log.debug "no pre-cached routes file at #{routes_file}. building."
      parse_map_and_dump_routes(map_file, routes_file)
      log.debug "dumped routes file"
    end

    routes = Routes.load(routes_file)

    solver = self.new(routes)
    solver.run
  end

  def self.parse_map_and_dump_routes(map_file, routes_file)
    map = Map.load(map_file)
    routes = map.build_routes(routes_file)
    routes.dump
  end

  def initialize(routes)
    @routes = routes
  end

  def run
    log.debug "Running solver"

    binding.pry

    # TODO update to not need map!

    starting_path = Path.initial(map, map.starting_location, map.keys)
    @open_set = SortedSet.new([starting_path])
    @best_for_location_and_collected_keys = Hash.new {|h, key| h[key] = {}}

    i = 1
    while !@open_set.empty?
      current_path = @open_set.first

      log.debug "Iteration #{i}, #{@open_set.size} paths, first one size #{current_path.steps} with #{current_path.collected_keys.size} collected keys"

      binding.pry if i == 200

      if current_path.complete?
        return current_path
      end

      @open_set.delete(current_path)

      # TODO(maybe) re-incorporate something from algorithm about only tracking one item per "location"
      # (where here "location" is maybe the set of seen items, or set of seen items + location?)

      current_path.next_paths.each do |next_path|
        current_best = @best_for_location_and_collected_keys[next_path.location][next_path.collected_keys]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_location_and_collected_keys[next_path.location][next_path.collected_keys] = next_path
        end
      end

      i += 1
    end

    nil
  end
end
