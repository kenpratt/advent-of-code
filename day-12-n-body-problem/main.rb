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
  foo = [simulation.dump_state]
  while !found_repeat && steps < max_steps
    steps += 1
    simulation.step!
    log.debug "step #{steps}" if steps % 5000 == 0
    #binding.pry if steps == 100
    snapshot = simulation.dump_state
    found_repeat = hist.repeat?(snapshot)
    hist.add!(snapshot) unless found_repeat
    foo << snapshot
  end
  binding.pry
  steps
end

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
    $min = 100
    $max = -100
    $fuzz = Set.new
  end

  def step!
    @combinations.each do |idx1, idx2|
      @moons[idx1].gravitate!(@moons[idx2])
    end
    @moons.each do |moon|
      moon.apply_velocity!
    end
    $fuzz << total_energy
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

def update_seen(val)
  $min = val if val < $min
  $max = val if val > $max
end

class Moon
  attr_accessor :px, :py, :pz, :vx, :vy, :vz

  def initialize(position)
    @px = position[:x]
    @py = position[:y]
    @pz = position[:z]
    @vx = 0
    @vy = 0
    @vz = 0
  end
    
  def potential_energy
    @px.abs + @py.abs + @pz.abs
  end

  def kinetic_energy
    @vx.abs + @vy.abs + @vz.abs
  end

  def total_energy
    potential_energy * kinetic_energy
  end

  def gravitate!(other)
    dx = (@px <=> other.px)
    @vx -= dx
    other.vx += dx

    dy = (@py <=> other.py)
    @vy -= dy
    other.vy += dy

    dz = (@pz <=> other.pz)
    @vz -= dz
    other.vz += dz

    update_seen(@vx)
    update_seen(@vy)
    update_seen(@vz)
    update_seen(other.vx)
    update_seen(other.vy)
    update_seen(other.vz)
  end

  def apply_velocity!
    @px += @vx
    @py += @vy
    @pz += @vz
    update_seen(@px)
    update_seen(@py)
    update_seen(@pz)
  end

  def dump_state
    [@px, @py, @pz, @vx, @vy, @vz]
  end

  def to_s
    sprintf("pos=<x=%3d, y=%3d, z=%3d>, vel=<x=%3d, y=%3d, z=%3d>", @px, @py, @pz, @vx, @vy, @vz)
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