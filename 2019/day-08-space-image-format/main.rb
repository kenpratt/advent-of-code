require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

Layer = Struct.new(:pixels, :width, :height) do
  def count_pixels(value)
    pixels.count(value)
  end

  def merge(other)
    new_pixels = self.pixels.zip(other.pixels).map do |p1, p2|
      p1 == 2 ? p2 : p1
    end
    Layer.new(new_pixels, width, height)
  end

  def to_grid
    pixels.each_slice(width).to_a
  end

  def to_s
    to_grid.map {|row| row.map {|s| render_pixel(s)}.join('')}.join("\n")
  end

  def render_pixel(p)
    case p
    when 0
      ' '
    when 1
      '#'
    when 2
      '%'
    else
      raise "Unknown pixel value: #{p}"
    end
  end
end

def process_input(input_str, width, height)
  pixels = input_str.each_char.map(&:to_i)
  pixels.each_slice(width * height).map do |layer_pixels|
    Layer.new(layer_pixels, width, height)
  end.to_a
end

def flatten_layers(layers)
  layers.reduce do |l1, l2| 
    l1.merge(l2)
  end
end

def part1(layers)
  layer = layers.min_by {|l| l.count_pixels(0)}
  layer.count_pixels(1) * layer.count_pixels(2)
end

def part2(layer)
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