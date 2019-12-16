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

def simulate_until_repeat(moon_positions, max_steps)
  simulation = Simulation.new(moon_positions)
  hist = History.new
  found_repeat = false
  steps = 0
  snapshot = simulation.dump_state
  hist.add!(simulation.dump_state)
  while !found_repeat && steps < max_steps
    steps += 1
    simulation.step!
    log.debug "step #{steps}" if steps % 5000 == 0
    #binding.pry if steps == 100
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
    @data = SortedSet.new
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
      Moon.new(position)
    end
    @combinations = (0...@moons.size).to_a.combination(2).to_a
  end

  def step!
    @combinations.each do |idx1, idx2|
      @moons[idx1].gravitate!(@moons[idx2])
    end
    @moons.each do |moon|
      moon.apply_velocity!
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
    @moons.flat_map(&:dump_state)
  end
end

class Moon
  attr_reader :data

  def initialize(position)
    @data = [position[:x], position[:y], position[:z], 0, 0, 0]
  end
    
  def potential_energy
    @data[0].abs + @data[1].abs + @data[2].abs
  end

  def kinetic_energy
    @data[3].abs + @data[4].abs + @data[5].abs
  end

  def total_energy
    potential_energy * kinetic_energy
  end

  def gravitate!(other)
    gravitate_helper(other, 0, 3) # x
    gravitate_helper(other, 1, 4) # y
    gravitate_helper(other, 2, 5) # z
  end

  private def gravitate_helper(other, pos_idx, vel_idx)
    p1 = @data[pos_idx]
    p2 = other.data[pos_idx]
    if p1 < p2
      @data[vel_idx] += 1
      other.data[vel_idx] -= 1
    elsif p1 > p2
      @data[vel_idx] -= 1
      other.data[vel_idx] += 1
    end
  end

  def apply_velocity!
    @data[0] += @data[3] # x
    @data[1] += @data[4] # y
    @data[2] += @data[5] # z
  end

  def dump_state
    clone
  end

  def to_s
    sprintf("pos=<x=%3d, y=%3d, z=%3d>, vel=<x=%3d, y=%3d, z=%3d>", *@data)
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