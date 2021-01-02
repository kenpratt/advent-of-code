require_relative 'droid'

Door = Struct.new(:room, :direction)

ITEMS_TO_AVOID = Set.new([
  'giant electromagnet',
  'escape pod',
  'infinite loop',
  'molten lava',
  'photons',
])

ROOM_WITH_WEIGHT_SENSOR = 'Pressure-Sensitive Floor'

class Solver
  attr_reader :droid

  def self.run(program)
    droid = Droid.start(program)
    self.new(droid).run
  end

  def initialize(droid)
    @droid = droid
    @halt = false
  end

  def run
    while !@halt do
      puts "\nTick".red
      puts "  Location: #{droid.current_room}".red
      puts ("  Doors to explore: " + doors_to_explore.map(&:to_s).join(', ')).red
      puts
      tick
    end

    droid.airlock_password
  end

  def tick
    if items_to_pick_up.any?
      pick_up_an_item
    elsif doors_to_explore.any?
      explore_a_door
    elsif droid.map.rooms.has_key?(ROOM_WITH_WEIGHT_SENSOR)
      solve_weight_sensor
    else
      binding.pry
    end
  end

  def items_to_pick_up
    droid.current_room.items.reject {|item| ITEMS_TO_AVOID.include?(item)}
  end

  def pick_up_an_item
    item = items_to_pick_up.first
    puts "Picking up #{item}".light_red
    droid.take(item)
  end

  def doors_to_explore
    droid.map.unexplored_doors
  end

  def explore_a_door
    door = doors_to_explore.first
    puts "Exploring a new door: #{door}".light_red
    move_to_room(door.room)
    move_in_direction(door.direction)
  end

  def move_to_room(destination)
    puts "Moving to: #{destination}".light_red
    if droid.current_room.name != destination
      steps = droid.map.calculate_route(droid.current_room.name, destination)
      puts "  Route: #{steps.inspect}".light_red
      steps.each do |direction|
        move_in_direction(direction)
      end
    end
  end

  def move_in_direction(direction)
    puts "Moving #{direction}".light_red
    droid.move(direction)
  end

  def solve_weight_sensor
    puts "Trying to activate the weight sensor".light_red
    move_to_room(ROOM_WITH_WEIGHT_SENSOR)

    items = droid.inventory
    item_subsets = (1...items.size).to_a.reverse.map {|n| items.combination(n).to_a}.flatten(1) 

    while item_subsets.any? && droid.airlock_password.nil?
      subset = item_subsets.shift
      adjust_inventory(subset)

      puts "Inventory: #{droid.inventory.join(', ')}".light_red
      move_to_room(ROOM_WITH_WEIGHT_SENSOR)
    end

    puts "Solved weight sensor!! Airlock password: #{droid.airlock_password.inspect}".red
    @halt = true
  end

  def adjust_inventory(wanted_inventory)
    current_inventory = droid.inventory
    to_drop = current_inventory - wanted_inventory
    to_pick_up = wanted_inventory - current_inventory
    puts "Dropping: #{to_drop.join(', ')}".light_red
    puts "Picking up: #{to_pick_up.join(', ')}".light_red
    to_drop.each {|item| droid.drop(item)}
    to_pick_up.each {|item| droid.take(item)}
    if droid.inventory.sort != wanted_inventory.sort
      binding.pry
    end
  end
end
