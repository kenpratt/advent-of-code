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

    @current_room = room

    if existing_room = @map.value(location)
      if existing_room != room
        binding.pry
      end
    else
      @map.paint!(room.location, room)
    end
  end

  def to_s(width=19)
    rows = @map.bounds.rendered_cells do |l|
      lines_for_location(l, width)
    end

    rows.map do |row|
      row[0].zip(*row[1..-1]).map {|a| a.join('')}.join("\n")
    end.flatten.join("\n")
  end

  def lines_for_location(location, width)
    inner_width = width - 2
    trunc_width = inner_width - 2
    centre = width / 2
    top_bottom_template = '+' + ('-' * inner_width) + '+'

    name = nil
    desc1 = nil
    desc2 = nil
    items1 = nil
    items2 = nil

    has_top_door = false
    has_bottom_door = false
    has_left_door = false
    has_right_door = false

    room = @map.value(location)
    if room.nil?
      return [' ' * width] * 7
    end

    has_top_door = room.doors.include?('north')
    has_bottom_door = room.doors.include?('south')
    has_left_door = room.doors.include?('west')
    has_right_door = room.doors.include?('east')

    top = top_bottom_template.clone
    if has_top_door
      top[centre-1] = ' '
      top[centre] = ' '
      top[centre+1] = ' '
    end

    bottom = top_bottom_template.clone
    if has_bottom_door
      bottom[centre-1] = ' '
      bottom[centre] = ' '
      bottom[centre+1] = ' '
    end

    left_door = has_left_door ? ' ' : '|'
    right_door = has_right_door ? ' ' : '|'

    name = room.name
    desc1 = room.description[0..trunc_width]
    desc2 = room.description[trunc_width..-1]
    items = room.items.join(', ')
    items1 = items[0..trunc_width]
    items2 = items[trunc_width..-1]
    
    row1 = '| ' + trunc_center(name, trunc_width) + ' |'
    row2 = '| ' + trunc_center(desc1, trunc_width) + ' |'
    row3 = left_door + ' ' + trunc_center(desc2, trunc_width) + ' ' + right_door
    row4 = '| ' + trunc_center(items1, trunc_width) + ' |'
    row5 = '| ' + trunc_center(items2, trunc_width) + ' |'

    [
      top,
      row1,
      row2,
      row3,
      row4,
      row5,
      bottom,
    ]
  end

  def trunc_center(str, width)
    if str.nil? || str.empty?
      ' ' * width
    elsif str.size >= width
      str[0, width]
    else
      extra = width - str.size
      half = extra / 2
      (' ' * half) + str + (' ' * (extra - half))
    end
  end
end

Room = Struct.new(:location, :name, :description, :doors, :items)
