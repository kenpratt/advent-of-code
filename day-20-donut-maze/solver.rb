require_relative 'map'
require_relative 'grid'

class Path
  attr_reader :routes, :location, :distance, :f_score, :g_score, :h_score

  def self.initial(routes, entrance)
    self.new(routes, entrance, 0)
  end

  def initialize(routes, location, distance)
    @routes = routes
    @location = location
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

    1
  end

  def next_paths
    routes.all_from(@location).map do |route|
      follow_route(route)
    end
  end

  def follow_route(route)
    next_location = route.to
    next_distance = @distance + route.distance
    self.class.new(@routes, next_location, next_distance)
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

    starting_path = Path.initial(@routes, @routes.entrance)
    @open_set = SortedSet.new([starting_path])
    @best_for_location = {}

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
        current_best = @best_for_location[next_path.location]

        if current_best.nil? || next_path.g_score < current_best.g_score
          @open_set << next_path
          @best_for_location[next_path.location] = next_path
        end
      end

      i += 1
    end

    binding.pry
    nil
  end
end
