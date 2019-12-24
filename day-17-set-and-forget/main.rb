require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

require_relative 'computer'
require_relative 'grid'

INPUT_FILE = File.join(__dir__, 'input.txt')

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
  
  solver = Solver.new(screen)
  solver.find_routes
  binding.pry

  input_lines = [
    "A,A,B,A,A,B",
    "L,10",
    "R,6",
    "L,2",
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

  binding.pry 

  result
end

class Solver
  attr_reader :screen

  def initialize(screen)
    @screen = screen
  end

  def find_routes
    # basic idea -- find all possible routes with array of turn/move instructions like R,10,L,5,etc.
    # then once I have the arrays, try to write a solver than breaks them into chunks.
    # 
    # more concretely:
    # - while not off screen / still have unvisited options
    # - figure out possible directions (left/right/straight, don't allow turning around)
    # - for "straight", add to last direction/instruction
    # - branch at intersections
    #
    # might want a subclass or something for this... or a functional thing. otherwise would have to clone the class a whole bunch.
    # 
    # try left + 1, right + 1, and straight + 1. filter out the neighbour behind the robot.
    #options = position.neighbours.select {|dir, n| screen.grid.value(n) == '#'}
    # figure out which of these are left/right/straight given current orientation
    # branch on them (recursive call?)

    # TODO need to track visited locations to not end up in loops that go on too long? or just make a max instruction count, maybe simpler?
    # recursive solution?

    initial_pointer = @screen.find_vacuuum_robot
    initial_path = Path.initial(initial_pointer, Set.new(screen.scaffold_locations))

    completed_paths = []
    in_progress_paths = SortedSet.new([initial_path])

    i = 0
    while in_progress_paths.any?
      log.info "path discovery iteration #{i} - #{in_progress_paths.size} in progress, #{completed_paths.size} completed" if i % 100 == 0
      new_in_progress_paths = []

      path = in_progress_paths.first
      in_progress_paths.delete(path)

      follow_path = true
      while follow_path
        if path.complete?
          log.info "completed a path! #{path}"
          completed_paths << path
          follow_path = false
        else
          branches = path.branches
          case branches.size
          when 0
            log.info "incomplete path: #{path.visited.size} #{path.pointer.position}"
            follow_path = false
          when 1
            path.follow_branch!(*branches[0])
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
    self.new(pointer, Set.new(to_visit), Set.new, [], 0)
  end

  def initialize(pointer, to_visit, visited, instructions, repeats)
    @pointer = pointer
    @to_visit = to_visit
    @visited = visited
    @instructions = instructions
    @repeats = 0
  end

  def current_position
    pointer.position
  end

  def complete?
    @to_visit == @visited
  end

  def branches
    options = [
      [:advance, @pointer.advance],
      [:left, @pointer.turn_left.advance],
      [:right, @pointer.turn_right.advance],
    ]
    options.select do |instruction, next_pointer|
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
      log.info "not ready for exit: #{@visited.size} #{@repeats.size}"
      false
    elsif next_position == BACK_TO_START && next_orientation == :up
      log.info "avoiding going back to entrance"
      false
    else
      true
    end
  end

  def follow_branch!(instruction, next_pointer)
    @pointer = next_pointer
    if @visited.include?(next_pointer.position)
      @repeats += 1
    else
      @visited << next_pointer.position
    end
    @instructions << instruction
  end

  def clone_branches(branches)
    branches.each_with_index.map do |branch, i|
      if i < (branches.size - 1)
        clone_and_move(*branch)
      else
        follow_branch!(*branch)
        self
      end
    end
  end

  def clone_and_move(instruction, next_pointer)
    already_visited = @visited.include?(next_pointer.position)
    self.class.new(
      next_pointer,
      @to_visit,
      already_visited ? @visited.clone : @visited.clone + [next_pointer.position],
      @instructions + [instruction],
      already_visited ? @repeats + 1 : @repeats,
    )
  end

  def score
    # num instructions so far + how many places remaining
    # but weight instructions heigher than nodes left to visit,
    # to try to avoid loops
    @instructions.size + @to_visit.size - @visited.size + @repeats * 5
  end

  def <=>(other)
    self.score <=> other.score
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
