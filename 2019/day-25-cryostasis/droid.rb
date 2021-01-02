require_relative 'computer'
require_relative 'map'

class Droid
  attr_reader :computer, :map, :current_room, :inventory, :airlock_password

  def self.start(program)
    droid = self.new(program)
    droid.start
    droid
  end

  def initialize(program)
    @computer = IntcodeComputer.new(program, [], "droid")
    @map = Map.new
    @current_room = nil
    @inventory = []
    @airlock_password = nil
  end

  def start
    output = run_computer
    move_output = parse_move_output(output)
    raise "Unexpected move output" if move_output.size != 1 || !move_output[0][1].nil?
    @current_room = move_output[0][0]
    @map.add_new_room(@current_room, nil)
  end

  def run_computer
    @computer.run
    output_raw = @computer.clear_output
    output = output_raw.map(&:chr).join('')
    puts output.blue
    output
  end

  def send_command(str)
    arr = str.each_char.map(&:ord) + [10]
    @computer.add_input_arr(arr)
    run_computer
  end

  def north
    move('north')
  end

  def south
    move('south')
  end

  def east
    move('east')
  end

  def west
    move('west')
  end

  def move(direction)
    if !@current_room.doors.include?(direction)
      raise "Invalid command, can't move #{direction}"
    end

    output = send_command(direction)
    move_output = parse_move_output(output)

    previous_room = @current_room
    destination_room, action = move_output[0]
    @current_room = @map.explored_door(previous_room, direction, destination_room)

    if !action.nil? || move_output.size > 1
      if action && action.include?('ejected back to') && move_output.size == 2
        # forced move back
        second_room, second_action = move_output[1]
        if previous_room.name == second_room.name && second_action.nil?
          @current_room = previous_room
        else
          binding.pry
        end
      elsif action && action.include?('Analysis complete!') && move_output.size == 1
        if action =~ /You should be able to get in by typing (\d+) on the keypad at the main airlock/
          @airlock_password = $1
        else
          binding.pry
        end
      else
        binding.pry
      end
    end
  end

  MOVE_OUTPUT_CHUNK = "(\n\n\=\= (.*) \=\=\n(.*)\n\n(Doors here lead:\n([a-z\\-\n ]+?)\n\n)?(Items here:\n([a-z\\-\n ]+?)\n\n)?((.+)\n\n)?)"
  RE_NORMAL_MOVE_OUTPUT = /\A\n#{MOVE_OUTPUT_CHUNK}Command\?\n\z/
  RE_MULTI_MOVE_OUTPUT = /\A\n#{MOVE_OUTPUT_CHUNK}#{MOVE_OUTPUT_CHUNK}Command\?\n\z/
  RE_MOVE_OUTPUT_CHUNK = /\A#{MOVE_OUTPUT_CHUNK}\z/
  RE_NO_COMMAND_MOVE_OUTPUT = /\A\n#{MOVE_OUTPUT_CHUNK}([\S\s]+)\z/

  def parse_move_output(str)
    if str =~ RE_NORMAL_MOVE_OUTPUT
      [parse_move_chunk($1)]
    elsif str =~ RE_MULTI_MOVE_OUTPUT
      chunk_1 = $1
      chunk_2 = $10
      [parse_move_chunk(chunk_1), parse_move_chunk(chunk_2)]
    elsif str =~ RE_NO_COMMAND_MOVE_OUTPUT
      leftovers = $10.strip
      room, action = parse_move_chunk($1)
      if !action.nil?
        binding.pry
      end
      [[room, leftovers]]
    else
      binding.pry
    end
  end

  def parse_move_chunk(str)
    if str =~ RE_MOVE_OUTPUT_CHUNK
      name = $2
      description = $3
      doors_str = $5 || ''
      items_str = $7 || ''
      action_str = $9
      doors = doors_str.split("\n").map {|s| s.sub(/^\- /, '')}
      items = items_str.split("\n").map {|s| s.sub(/^\- /, '')}
      [Room.new(@map, name, description, doors, items), action_str]
    else
      binding.pry
    end
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
end