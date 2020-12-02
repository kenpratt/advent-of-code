require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :routes, :robot_locations, :to_visit, :collected_keys, :distance, :f_score, :g_score, :h_score

  def self.initial(routes, entrances, to_visit)
    # start each robot at the entrance
    robot_locations = entrances.map {|e| [e, e]}.to_h
    self.new(routes, robot_locations, to_visit, Set.new, 0)
  end

  def initialize(routes, robot_locations, to_visit, collected_keys, distance)
    @routes = routes
    @robot_locations = robot_locations
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
    max_per_entrance = Hash.new(0)
    @robot_locations.each do |entrance, location|
      @to_visit.each do |to|
        route = routes.get(entrance, location, to)
        if route && route.distance < max_per_entrance[entrance]
          max_per_entrance[entrance] = route.distance
        end
      end
    end
    @robot_locations.map {|entrance, _| max_per_entrance[entrance]}.sum
  end

  def next_paths
    # calculate possible path to each unvisited key that is reachable
    # and return new paths including the distance walked to them
    out = []

    @robot_locations.each do |entrance, location|
      @to_visit.each do |next_location|
        route = routes.get(entrance, location, next_location)
        if route && should_follow_route?(route)
          new_path = follow_route(entrance, route)
          out << new_path
        end
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
  def follow_route(entrance, route)
    next_robot_locations = robot_locations.merge(entrance => route.to)
    next_to_visit = @to_visit - [route.to]
    next_collected_keys = @collected_keys + [route.to]
    next_distance = @distance + route.distance
    self.class.new(@routes, next_robot_locations, next_to_visit, next_collected_keys, next_distance)
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

    starting_path = Path.initial(@routes, @routes.entrances, @routes.to_visit)
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
        current_best = @best_for_location_and_collected_keys[next_path.robot_locations][next_path.collected_keys]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_location_and_collected_keys[next_path.robot_locations][next_path.collected_keys] = next_path
        end
      end

      i += 1
    end

    binding.pry
    nil
  end
end
