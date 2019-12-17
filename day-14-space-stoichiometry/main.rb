require 'pry'

require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(input_str)
  input_str.split("\n").map {|l| parse_line(l)}
end

def parse_line(str)
  input_strs, output_str = str.split(' => ')
  inputs = input_strs.split(', ').map {|s| parse_component(s)}
  output = parse_component(output_str)
  Recipe.new(inputs, output)
end

def parse_component(str)
  if str =~ /\A(\d+) (\w+)\z/
    Component.new($1.to_i, $2)
  else
    raise "Unexpected component string: #{str}"
  end
end

def calculate_ore_necessary_for_one_fuel(recipes)
  calc = InputCalculator.new(recipes, 'ORE', 'FUEL', 1)
  calc.run
end

Recipe = Struct.new(:inputs, :output)
Component = Struct.new(:quantity, :name)

class InputCalculator
  def initialize(recipes, input_name, output_name, output_quantity)
    if recipes.map(&:output).map(&:name).uniq.size != recipes.size
      raise "Recipes do not have unique outputs"
    end
    @input_name = input_name

    # build recipe lookup table
    @recipe_for_output = recipes.map {|r| [r.output.name, r]}.to_h

    # build recipe dependency graph
    @recipe_dependencies = Hash.new {|h,k| h[k] = Set.new}
    recipes.each do |recipe|
      recipe.inputs.each do |input|
        @recipe_dependencies[input.name] << recipe.output.name
      end
    end

    # track items we're done with
    @processed = Set.new

    # current list of stuff
    @ingredients = Hash.new(0)
    @ingredients[output_name] = output_quantity
  end

  def run
    while !finished?
      step
    end
    @ingredients[@input_name]
  end

  def finished?
    @ingredients.size == 1 && @ingredients.has_key?(@input_name)
  end

  def step
    name = choose_ingredient_to_deconstruct
    return nil unless name

    log.debug "have: " + @ingredients.map {|k, v| "#{v} #{k}"}.join(', ')
    quantity = @ingredients.delete(name)
    necessary_inputs = calculate_inputs(name, quantity)
    log.debug "convert: #{quantity} #{name} => " + necessary_inputs.map {|k, v| "#{v} #{k}"}.join(', ')
    if @processed.include?(name)
      raise "Processing the same name twice: #{name}"
    end
    @processed << name
    @ingredients.merge!(necessary_inputs) {|_key, val1, val2| val1 + val2}
  end

  def choose_ingredient_to_deconstruct
    names = @ingredients.keys - [@input_name]
    return names[0] if names.size == 1

    # ensure we don't process any ingredient that still has dangling dependencies
    names.find {|n| @recipe_dependencies[n].subset?(@processed)}
  end

  def calculate_inputs(output_name, output_quantity)
    recipe = @recipe_for_output[output_name]
    multiple = (output_quantity / recipe.output.quantity.to_f).ceil
    recipe.inputs.map {|i| [i.name, i.quantity * multiple]}.to_h
  end
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