require_relative 'grid'

class Map
  attr_reader :rooms, :unexplored_doors

  def initialize
    @rooms = {}
    @unexplored_doors = []
  end

  def add_new_room(room, ignore_door)
    raise "Already have room #{room.name}" if @rooms.has_key?(room.name)
    @rooms[room.name] = room
    room.doors.each do |direction|
      # exclude the door we just came through
      if direction != ignore_door
        @unexplored_doors << UnexploredDoor.new(room.name, direction)
      end
    end
  end

  def explored_door(origin_room, direction, destination_room)
    # did we find a new room?
    if @rooms.has_key?(destination_room.name)
      # use existing stateful object
      existing_destination_room = @rooms[destination_room.name]
      existing_destination_room.validate(destination_room)
      destination_room = existing_destination_room
    else
      add_new_room(destination_room, invert_direction(direction))
    end

    # remove the door we explored
    @unexplored_doors.delete(UnexploredDoor.new(origin_room.name, direction))

    # make the link between the two rooms
    origin_room.set_door_destination(direction, destination_room.name)
    destination_room.set_door_destination(invert_direction(direction), origin_room.name)

    # return the new-or-existing object
    destination_room
  end

  def calculate_route(origin, destination)
    route = PathfindingAStar.find_path(@rooms[origin], @rooms[destination]) {true}
    binding.pry if route.nil?
    route.map(&:first)
  end  
end

class Room
  attr_reader :name, :description, :doors
  attr_accessor :items

  def initialize(map, name, description, doors, items)
    @map = map
    @name = name
    @description = description
    @doors = doors
    @items = items
    @door_map = {}
  end

  def set_door_destination(direction, room_name)
    if @doors.include?(direction)
      @door_map[direction] = room_name
    else
      raise "Nonsensical door: #{direction}"
    end
  end

  def validate(fresh_room)
    if @name != fresh_room.name || @doors != fresh_room.doors || @items != fresh_room.items
      binding.pry
    end
  end

  def manhattan_distance(destination_room)
    1 # fake heuristic for A*
  end

  def neighbours
    @door_map.map do |direction, room_name|
      [direction, @map.rooms[room_name]]
    end
  end

  def to_s
    "#{name} " + doors.map {|d| "[#{d} -> " + (@door_map[d] || '?') + ']'}.join(', ')
  end
end

UnexploredDoor = Struct.new(:room, :direction) do
  def to_s
    "[#{room} | #{direction}]"
  end
end

def invert_direction(direction)
  case direction
  when 'north' then 'south'
  when 'south' then 'north'
  when 'west' then 'east'
  when 'east' then 'west'
  else raise "Unknown direction: #{direction}"
  end
end
