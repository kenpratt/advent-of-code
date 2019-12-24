require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'
require_relative 'grid'

INPUT_FILE = File.join(__dir__, 'input.txt')
ROUTES_FILE = File.join(__dir__, 'routes.txt')

def parse_program(input_str)
  input_str.strip.split(',').map(&:to_i)
end

def start_program(program)
  computer = IntcodeComputer.new(program, [])
  computer.run
end

def sum_alignment_parameters(screen_contents_str)
  screen = Screen.new(screen_contents_str)
  intersections = screen.scaffold_intersections
  intersections.map {|c| c.x * c.y}.sum
end

def explore_whole_scaffold(program)
  program[0] = 2
  computer = IntcodeComputer.new(program, [])
  computer.run
  output = computer.clear_output
  screen_str = output.map(&:chr).join('')
  puts screen_str
  screen = Screen.new(screen_str)
  
  run_solver = false
  if run_solver
    solver = Solver.new(screen)
    paths = solver.find_routes(200)
    File.open(ROUTES_FILE, 'w') do |f|
      paths.sort_by {|p| p.instructions.size}.each do |path|
        f << path.instructions.join(',') 
        f << "\n"
      end
    end
  end

  routes = File.readlines(ROUTES_FILE).map(&:strip).map {|l| l.split(',')}

  data = routes.map do |route|
    [route, analyze_route(route)]
  end

  route, solutions = data.find {|route, solutions| solutions.any?}
  solution = solutions.first

  # A,A,B,B,C,B,C,B,C,A
  # A: L,10,L,10,R,6
  # B: R,12,L,12,L,12
  # C: L,6,L,10,R,12,R,12
  main_routine, functions = *solution

  input_lines = [
    main_routine,
    *functions,
    "y",
  ]
  input_arr = input_lines.map {|s| s + "\n"}.join('').each_char.map(&:ord)

  computer.add_input_arr(input_arr)
  computer.run

  output = computer.clear_output

  result = nil
  if output.last > 255
    result = output.pop
  end

  puts output.map(&:chr).join('')

  result
end

def analyze_route(route_arr)
  # use x between rotation and move command to make analysis easier but keep char mem limit
  route_str = route_arr.each_slice(2).map {|a| a.join('x')}.join(',')

  output = []
  guess_next_function(route_str, ['A', 'B', 'C'], [], output)
  output
end

def guess_next_function(route_str, functions_left, function_strs, output)
  if functions_left.empty?
    if route_str =~ /\A[ABC,]+\z/ && route_str.size <= 20
      output << [route_str, function_strs.map {|s| s.gsub('x', ',')}]
    end
    return
  end

  function_name = functions_left[0]
  next_functions_left = functions_left[1..-1]

  route_arr = route_str.split(',')
  start_index = route_arr.find_index {|s| is_instruction?(s)}
  if start_index.nil?
    binding.pry
  end
  end_index = start_index

  while end_index < route_arr.size && is_instruction?(route_arr[end_index])
    function_str = route_arr[start_index..end_index].join(',')
    if function_str.size <= 20
      next_route_str = route_str.gsub(function_str, function_name)
      next_function_strs = function_strs + [function_str]
      guess_next_function(next_route_str, next_functions_left, next_function_strs, output)
    else
      break # no point continuing
    end
    end_index += 1
  end
end

def is_instruction?(s)
  s[0] == 'L' || s[0] == 'R'
end

class Solver
  attr_reader :screen

  def initialize(screen)
    @screen = screen
  end

  def find_routes(how_many_paths)
    initial_pointer = @screen.find_vacuuum_robot
    initial_path = Path.initial(initial_pointer, Set.new(screen.scaffold_locations))

    completed_paths = []
    in_progress_paths = SortedSet.new([initial_path])

    i = 0
    while in_progress_paths.any? && completed_paths.size < how_many_paths
      log.info "path discovery iteration #{i} - #{in_progress_paths.size} in progress, #{completed_paths.size} completed" if i % 100 == 0

      path = in_progress_paths.first
      in_progress_paths.delete(path)

      follow_path = true
      while follow_path
        if path.complete?
          log.info "completed a path! #{path.instructions.size}"
          completed_paths << path
          follow_path = false
        else
          branches = path.branches
          case branches.size
          when 0
            log.info "incomplete path: #{path.visited.size} #{path.pointer.position}"
            follow_path = false
          when 1
            # follow and override cost to 0 since it was the only option
            branch = branches[0]
            path.follow_branch!(branch[0], branch[1], 0)
            follow_path = true
          else
            in_progress_paths += path.clone_branches(branches)
            follow_path = false
          end
        end
      end

      i += 1
    end

    completed_paths
  end
end

class Path
  include Comparable

  attr_reader :pointer, :to_visit, :visited, :instructions, :repeats

  def self.initial(pointer, to_visit)
    self.new(pointer, Set.new(to_visit), Set.new, [], 0, 0)
  end

  def initialize(pointer, to_visit, visited, instructions, repeats, cost)
    @pointer = pointer
    @to_visit = to_visit
    @visited = visited
    @instructions = instructions
    @repeats = 0
    @cost = 0
  end

  def current_position
    pointer.position
  end

  def complete?
    @to_visit == @visited
  end

  def branches
    options = [
      [:advance, @pointer.advance, 0],
      # penalize turning at forks
      [:left, @pointer.turn_left.advance, UNNECESSARY_TURN_COST],
      [:right, @pointer.turn_right.advance, UNNECESSARY_TURN_COST],
    ]
    options.select do |instruction, next_pointer, cost|
      want_to_visit?(next_pointer)
    end
  end

  EXIT_NODE = Coordinate.new(28, 37)
  NUM_IN_EXIT = 10
  BACK_TO_START = Coordinate.new(20, 23)

  def want_to_visit?(next_pointer)
    next_position = next_pointer.position
    next_orientation = next_pointer.orientation
    return false unless @to_visit.include?(next_position)

    if next_position == EXIT_NODE && @visited.size < (@to_visit.size - NUM_IN_EXIT)
      #log.info "not ready for exit: #{@visited.size} #{@repeats.size}"
      false
    elsif next_position == BACK_TO_START && next_orientation == :up
      false
    else
      true
    end
  end

  def clone_branches(branches)
    branches.each_with_index.map do |branch, i|
      if i < (branches.size - 1)
        clone_and_follow(*branch)
      else
        follow_branch!(*branch)
      end
    end
  end

  def clone
    self.class.new(@pointer, @to_visit, @visited.clone, @instructions.clone, @repeats, @cost)
  end

  def clone_and_follow(instruction, next_pointer, cost)
    cloned = clone
    cloned.follow_branch!(instruction, next_pointer, cost)
  end

  def follow_branch!(instruction, next_pointer, cost)
    @pointer = next_pointer

    if @visited.include?(next_pointer.position)
      @repeats += 1
    else
      @visited << next_pointer.position
    end

    case instruction
    when :advance
      if @instructions.last.is_a?(Integer)
        @instructions[-1] += 1
      else
        @instructions << 1
      end
    when :left
      @instructions << 'L'
      @instructions << 1
    when :right
      @instructions << 'R'
      @instructions << 1
    else
      raise "Unknown instruction"
    end

    @cost += cost

    self
  end

  UNNECESSARY_TURN_COST = 4
  REPEAT_COST = 2

  def score
    # num instructions so far + how many places remaining
    # but weight instructions heigher than nodes left to visit,
    # to try to avoid loops
    @instructions.size + num_left_to_visit + (@repeats * REPEAT_COST) + @cost
  end

  def num_left_to_visit
    @to_visit.size - @visited.size
  end

  def <=>(other)
    self.score <=> other.score
  end

  def to_s
    "#{@instructions.join(',')} (#{instructions.size}) left: #{num_left_to_visit}, score: #{score}, repeats: #{@repeats}, cost: #{@cost}"
  end
end

class Screen
  attr_reader :grid

  def initialize(contents_str)
    rows = contents_str.split("\n").map {|s| s.each_char.to_a}
    @grid = StaticGrid.from_rows(rows)
  end

  def scaffold_locations
    @grid.cells_with_value('#')
  end

  def scaffold_intersections
    scaffold_locations.select do |coord|
      coord.neighbours.all? {|_, n| @grid.value(n) == '#'}
    end
  end

  def value(coord)
    @grid.value(coord)
  end

  def in_bounds?(coord)
    @grid.in_bounds?(coord)
  end

  def robot_char_to_orientation(char)
    case char
    when '^'
      :up
    when 'v'
      :down
    when '<'
      :left
    when '>'
      :right
    else
      raise "Unknown orientation: #{char}"
    end
  end

  def find_vacuuum_robot
    positions = @grid.cells_with_values(['^', 'v', '<', '>'])
    unless positions.size == 1
      raise "Couldn't locate robot"
    end
    position = positions[0]
    value = @grid.value(position)
    Pointer.new(position, robot_char_to_orientation(value))
  end
end
