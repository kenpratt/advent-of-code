require_relative 'computer'
require_relative 'grid'

class Droid
  attr_reader :computer, :map, :current_room, :inventory

  def self.start(program)
    droid = self.new(program)
    droid.start
    droid
  end

  def initialize(program)
    @computer = IntcodeComputer.new(program, [], "droid")
    @map = GrowableGrid.new
    @current_room = nil
    @inventory = []
  end

  def start
    output = run_computer
    handle_move_output(output, Coordinate.new(0, 0))
  end

  def run_computer
    @computer.run
    output_raw = @computer.clear_output
    output = output_raw.map(&:chr).join('')
    puts output
    output
  end

  def send_command(str)
    arr = str.each_char.map(&:ord) + [10]
    @computer.add_input_arr(arr)
    run_computer
  end

  def north
    move('north', :up)
  end

  def south
    move('south', :down)
  end

  def east
    move('east', :right)
  end

  def west
    move('west', :left)
  end

  def take(item)
    output = send_command("take #{item}")
    if output.strip =~ /^You take the #{item}\.\n\nCommand\?$/
      @inventory << item
      @current_room.items -= [item]
    else
      binding.pry
    end
  end

  def drop(item)
    output = send_command("drop #{item}")
    if output.strip =~ /^You drop the #{item}\.\n\nCommand\?$/
      @inventory -= [item]
      @current_room.items << item
    else
      binding.pry
    end
  end

  def read_inventory
    send_command("inv")
  end

  def move(direction, coord_direction)
    if !@current_room.doors.include?(direction)
      raise "Invalid command, can't move #{direction}"
    end

    output = send_command(direction)

    new_location = @current_room.location.move(coord_direction)
    handle_move_output(output, new_location)
  end

  def handle_move_output(str, location)
    room = if str.strip =~ /^\=\= (.*) \=\=\n(.*)\n\n(Doors here lead:\n([a-z\-\n ]+?)\n\n)?(Items here:\n([a-z\-\n ]+?)\n\n)?Command\?$/
      name = $1
      description = $2
      doors_str = $4 || ''
      items_str = $6 || ''
      doors = doors_str.split("\n").map {|s| s.sub(/^\- /, '')}
      items = items_str.split("\n").map {|s| s.sub(/^\- /, '')}
      Room.new(location, name, description, doors, items)
    else
      binding.pry
    end

    puts room
    @current_room = room
    @map.paint!(room.location, room)
  end
end

Room = Struct.new(:location, :name, :description, :doors, :items)