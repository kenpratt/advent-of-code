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
    initial_path = Path.new(initial_pointer, [])

    completed_paths = []
    in_progress_paths = [initial_path]

    i = 0
    while in_progress_paths.any?
      log.info "path discovery iteration #{i} - #{in_progress_paths.size} in progress, #{completed_paths.size} completed"
      new_in_progress_paths = []

      in_progress_paths.each do |path|
        options = path.forward_directions
        valid_options = options.select do |next_path|
          next_position = next_path.current_position
          screen.in_bounds?(next_position) && screen.value(next_position) == '#'
        end
    
        if valid_options.any?
          new_in_progress_paths += valid_options
        else
          completed_paths << path
        end    
      end

      # try to avoid already visited nodes -- todo need to track visited nodes in path...

      #binding.pry
      in_progress_paths = new_in_progress_paths
      i += 1
    end

    completed_paths
  end
end

Path = Struct.new(:pointer, :instructions) do
  def current_position
    pointer.position
  end

  def forward_directions
    [
      Path.new(pointer.advance, instructions + [:advance]),
      Path.new(pointer.turn_left.advance, instructions + [:turn_left, :advance]),
      Path.new(pointer.turn_right.advance, instructions + [:turn_right, :advance]),
    ]
  end
end

class Screen
  attr_reader :grid

  def initialize(contents_str)
    rows = contents_str.split("\n").map {|s| s.each_char.to_a}
    @grid = StaticGrid.from_rows(rows)
  end

  def scaffold_intersections
    @grid.cells_with_value('#').select do |coord|
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
