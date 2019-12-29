require_relative 'map'
require_relative 'grid'

class RecursivePath
  attr_reader :routes, :bounds, :location, :level, :distance, :f_score, :g_score, :h_score

  @@shortest_distance_to_exit = nil
  @@shortest_distance_between_portals = nil

  def self.initial(routes, bounds, entrance)
    self.new(routes, bounds, entrance, 0, 0)
  end

  def initialize(routes, bounds, location, level, distance)
    @routes = routes
    @bounds = bounds
    @location = location
    @level = level
    @distance = distance

    update_scores
  end

  def complete?
    @location.last == 'ZZ'
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
    if complete?
      return 0
    end

    (0..@level).map {|l| h_score(l)}.sum
  end

  def h_score(level)
    if level == 0
      shortest_distance_to_exit
    elsif level == @level
      shortest_distance_to_portal
    else
      shortest_distance_between_portals
    end
  end

  def shortest_distance_to_exit
    return @@shortest_distance_to_exit if @@shortest_distance_to_exit
    @@shortest_distance_to_exit = @routes.routes_set.select do |route|
      route.to.last == 'ZZ'
    end.min_by(&:distance).distance
    @@shortest_distance_to_exit
  end

  def shortest_distance_between_portals
    return @@shortest_distance_between_portals if @@shortest_distance_between_portals
    @@shortest_distance_between_portals = @routes.routes_set.select do |route|
      route.from.last != 'AA' && route.to.last != 'ZZ' && !is_warp?(route)
    end.min_by(&:distance).distance
    @@shortest_distance_between_portals
  end

  def shortest_distance_to_portal
    @routes.all_from(@location).reject {|r| is_warp?(r)}.min_by(&:distance).distance
  end

  def next_paths
    out = []
    routes.all_from(@location).each do |route|
      if valid_route?(route)
        out << follow_route(route)
      end
    end
    out
  end

  def valid_route?(route)
    if is_warp?(route)
      false
    elsif @level == 0
      # outer edges aren't available on level 0
      route.to.last == 'ZZ' || !on_outer_edge?(route.to)
    else
      # exit is only available on first level
      route.to.last != 'ZZ'
    end
  end

  def follow_route(route)
    if route.to.last == 'ZZ'
      next_location = route.to
      next_level = @level
      next_distance = @distance + route.distance
      self.class.new(@routes, @bounds, next_location, next_level, next_distance)
    elsif is_warp?(route)
      raise 'Nonsense'
    else
      warp = @routes.all_from(route.to).find {|r| is_warp?(r)}
      next_location = warp.to
      next_level = @level + level_change(warp)
      next_distance = @distance + route.distance + warp.distance
      self.class.new(@routes, @bounds, next_location, next_level, next_distance)
    end
  end

  def is_warp?(route)
    route.from.last == route.to.last
  end

  def on_outer_edge?(location)
    x, y, _ = *location
    @bounds.left == x || @bounds.right == x || @bounds.top == y || @bounds.bottom == y
  end

  def level_change(route)
    if is_warp?(route)
      from_outer_edge = on_outer_edge?(route.from)
      to_outer_edge = on_outer_edge?(route.to)
      if from_outer_edge && !to_outer_edge
        -1
      elsif !from_outer_edge && to_outer_edge
        1
      else
        raise "Nonsense"
      end
    else
      0
    end
  end

  def <=>(other)
    self.f_score <=> other.f_score
  end
end

class RecursiveSolver
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
    build_bounds
  end

  def build_bounds
    x, y, _ = @routes.routes_set.first.from
    @bounds = Bounds.new(x, x, y, y)
    @routes.routes_set.each do |route|
      x, y, _ = route.from
      @bounds.expand!(Coordinate.new(x, y))

      x, y, _ = route.to
      @bounds.expand!(Coordinate.new(x, y))
    end
  end

  def run
    log.debug "Running solver"

    starting_path = RecursivePath.initial(@routes, @bounds, @routes.entrance)

    #starting_path = RecursivePath.new(@routes, @bounds, [17, 2, "XQ"], 10, 0)
    #starting_path = RecursivePath.new(@routes, @bounds, [19, 2, "WB"], 10, 0)
    #starting_path = RecursivePath.new(@routes, @bounds, [36, 13, "WB"], 9, 0)
    #starting_path = RecursivePath.new(@routes, @bounds, [42, 13, "ZH"], 9, 0)
    #starting_path = RecursivePath.new(@routes, @bounds, [27, 2, "CK"], 8, 0)
    #starting_path = RecursivePath.new(@routes, @bounds, [13, 8, "FD"], 0, 0)

    @open_set = SortedSet.new([starting_path])
    @best_for_level_and_location = Hash.new {|h,k| h[k] = {}}

    i = 1
    while !@open_set.empty?
      current_path = @open_set.first

      log.debug "Iteration #{i}, #{@open_set.size} paths, first one size #{current_path.distance}" if i % 100 == 0

      #binding.pry if i == 200

      if current_path.complete?
        return current_path
      end

      @open_set.delete(current_path)

      next_paths = current_path.next_paths
      next_paths.each do |next_path|
        current_best = @best_for_level_and_location[next_path.level][next_path.location]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_level_and_location[next_path.level][next_path.location] = next_path
        end
      end

      i += 1
    end

    binding.pry
    nil
  end
end
