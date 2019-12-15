require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.split("\n").map(&:strip).map {|s| line_to_h(s)}
end

def line_to_h(str)
  if str =~ /^<(.*)>$/
    parts = $1.strip.split(',').map(&:strip)
    parts.map do |part|
      if part =~ /^(.*)\=(.*)$/
        [$1.strip.to_sym, $2.strip.to_i]
      else
        raise "Unknown line part format: #{part}"
      end
    end.to_h
  else
    raise "Unknown line format: #{str}"
  end
end

def simulate(moon_positions, iterations)
  simulation = Simulation.new(moon_positions)
  iterations.times {simulation.step!}
  simulation
end

def simulate_until_repeat(moon_positions)
  simulation = Simulation.new(moon_positions)
  hist = History.new
  found_repeat = false
  steps = 0
  snapshot = simulation.dump_state
  hist.add!(simulation.dump_state)
  while !found_repeat
    steps += 1
    simulation.step!
    snapshot = simulation.dump_state
    found_repeat = hist.repeat?(snapshot)
    hist.add!(snapshot) unless found_repeat
  end
  steps
end

AXES = [:x, :y, :z]

class History
  attr_reader :data

  def initialize
    @data = []
  end

  def repeat?(state)
    @data.include?(state)
  end

  def add!(state)
    @data << state
  end
end

class Simulation
  attr_reader :moons

  def initialize(moon_positions)
    @moons = moon_positions.map do |position|
      Moon.new(position, {x: 0, y: 0, z: 0})
    end
  end

  def step!
    @moons.combination(2).each do |moon1, moon2|
      gravitate!(moon1, moon2)
    end
    @moons.each do |moon|
      apply_velocity!(moon)
    end
  end

  def gravitate!(moon1, moon2)
    AXES.each do |axis|
      v1 = moon1.position[axis]
      v2 = moon2.position[axis]
      if v1 < v2
        moon1.velocity[axis] += 1
        moon2.velocity[axis] -= 1
      elsif v1 > v2
        moon1.velocity[axis] -= 1
        moon2.velocity[axis] += 1
      end
    end
  end
  
  def apply_velocity!(moon)
    AXES.each do |axis|
      moon.position[axis] += moon.velocity[axis]
    end
  end

  def states_to_s
    @moons.map(&:to_s).join("\n")
  end

  def energy_to_s
    @moons.map(&:energy_to_s).join("\n")
  end

  def total_energy
    @moons.sum(&:total_energy)
  end

  def dump_state
    @moons.map(&:dump_state)
  end
end

Moon = Struct.new(:position, :velocity) do
  def potential_energy
    position.values.sum(&:abs)
  end

  def kinetic_energy
    velocity.values.sum(&:abs)
  end

  def total_energy
    potential_energy * kinetic_energy
  end

  def dump_state
    [position.clone, velocity.clone]
  end

  def to_s
    sprintf(
      "pos=<x=%3d, y=%3d, z=%3d>, vel=<x=%3d, y=%3d, z=%3d>",
      position[:x],
      position[:y],
      position[:z],
      velocity[:x],
      velocity[:y],
      velocity[:z],
    )
  end

  def energy_to_s
    sprintf(
      "pot:%3d; kin:%3d; total:%3d",
      potential_energy,
      kinetic_energy,
      total_energy,
    )
  end
end

def part1(input)
  nil
end

def part2(input)
  nil
end

def main
  input_str = File.read(INPUT_FILE)
  input = process_input(input_str)

  log.info "Part 1:"
  log.info measure{part1(input)}

  log.info "Part 2:"
  log.info measure{part2(input)}
end

if __FILE__ == $0
  main
end