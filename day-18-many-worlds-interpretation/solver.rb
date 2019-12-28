require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :routes, :location, :to_visit, :collected_keys, :distance, :f_score, :g_score, :h_score

  def self.initial(routes, location, to_visit)
    self.new(routes, location, to_visit, Set.new, 0)
  end

  def initialize(routes, location, to_visit, collected_keys, distance)
    @routes = routes
    @location = location
    @to_visit = to_visit

    @collected_keys = collected_keys
    @distance = distance

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
    @distance
  end

  def calculate_h_score
    if @to_visit.empty?
      return 0
    end

    # if @to_visit.size == 1
    #   return routes.get(@location, @to_visit.first).distance
    # end

    # if @to_visit.size == 2
    #   loc1 = @to_visit[0]
    #   loc2 = @to_visit[1]
    #   dist1 = routes.get(@location, loc1).distance
    #   dist2 = routes.get(@location, loc2).distance
    #   dist_between = routes.get(loc1, loc2).distance
    #   return dist_between + (dist1 < dist2 ? dist1 : dist2)
    # end

    # TODO MST
    #mst_distance = routes.minimum_spanning_tree_distance(@to_visit)
    #connections = @to_visit.map {|to| routes.get(@location, to).distance}.sort
    #mst_distance + connections[0] + connections[1]

    #routes.minimum_spanning_tree_distance([location] + @to_visit)

    # faster heuristic
    @to_visit.map {|to| routes.get(@location, to).distance}.max
  end

  def next_paths
    # calculate possible path to each unvisited key that is reachable
    # and return new paths including the distance walked to them
    out = []

    @to_visit.each do |next_location|
      route = routes.get(@location, next_location)
      if should_follow_route?(route)
        new_path = follow_route(route)
        out << new_path
      end
    end

    out
  end

  def should_follow_route?(route)
    # if we have all the necessary keys *and* we've visited
    # everything along the way (otherwise we should stop at those)
    route.have_necessary_keys?(@collected_keys) &&
      route.have_all_keys_along_route?(@collected_keys)
  end

  # TODO consruct a new Path at the next_location location:
  # - adding the distance to get there
  # - marking the new location as visited (removing from to_visit)
  # - and recalculating scores
  # - (need a clone func)
  def follow_route(route)
    next_location = route.to
    next_to_visit = @to_visit - [next_location]
    next_collected_keys = @collected_keys + [next_location]
    next_distance = @distance + route.distance
    self.class.new(@routes, next_location, next_to_visit, next_collected_keys, next_distance)
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

    starting_path = Path.initial(@routes, @routes.starting_value, @routes.to_visit)
    @open_set = SortedSet.new([starting_path])
    @best_for_location_and_collected_keys = Hash.new {|h, key| h[key] = {}}

    i = 1
    while !@open_set.empty?
      current_path = @open_set.first

      log.debug "Iteration #{i}, #{@open_set.size} paths, first one size #{current_path.distance} with #{current_path.collected_keys.size} collected keys" if i % 100 == 0

      #binding.pry if i == 200

      if current_path.complete?
        return current_path
      end

      @open_set.delete(current_path)

      next_paths = current_path.next_paths
      next_paths.each do |next_path|
        current_best = @best_for_location_and_collected_keys[next_path.location][next_path.collected_keys]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_location_and_collected_keys[next_path.location][next_path.collected_keys] = next_path
        end
      end

      i += 1
    end

    binding.pry
    nil
  end
end
