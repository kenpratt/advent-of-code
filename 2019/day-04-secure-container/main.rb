require_relative '../utils/log'
require_relative '../utils/profile'

def part1_try1(input)
  num_start, num_end = *input
  num_start.upto(num_end).count {|x| passes_tests(num_to_digits(x))}
end

def passes_tests(digits)
  found_repeat = false
  last = digits[0]
  digits[1..-1].each do |curr|
    return false if curr < last
    found_repeat = true if curr == last
    last = curr
  end
  found_repeat
end

def part1_try2(input)
  num_start, num_end = *input
  counter = SimpleCounter.new(num_start, num_end)
  counter.count_matches
end

class SimpleCounter
  def initialize(num_start, num_end)
    @num = num_start
    @num_end = num_end
    @digits = num_to_digits(num_start)
    @last_place = @digits.size - 1
  end

  def increment
    @num += 1
    place = @last_place
    while place >= 0
      if @digits[place] == 9
        @digits[place] = 0
        place = place - 1
      else
        @digits[place] += 1
        break
      end
    end
  end

  def count_matches
    matches = 0
    while @num <= @num_end
      matches += 1 if passes_tests(@digits)
      increment
    end
    matches
  end
end

def part1_try3(input)
  num_start, num_end = *input
  counter = SmartCounter.new(num_start, num_end, false)
  counter.count_matches
end

def part2(input)
  num_start, num_end = *input
  counter = SmartCounter.new(num_start, num_end, true)
  counter.count_matches
end

def passes_tests_require_one_double(digits)
  found_a_double = false
  repeat_length = 1
  last = digits[0]
  digits[1..-1].each do |curr|
    return false if curr < last
    if curr == last
      repeat_length += 1
    else
      if !found_a_double && repeat_length == 2
        found_a_double = true
      end
      repeat_length = 1
    end
    last = curr
  end
  if !found_a_double && repeat_length == 2
    found_a_double = true
  end  
  found_a_double
end

class SmartCounter
  def initialize(num_start, num_end, require_one_double)
    @digits = num_to_digits(num_start)
    @digits_end = num_to_digits(num_end)
    @require_one_double = require_one_double
    @last_place = @digits.size - 1
  end

  def increment
    increment_place(@last_place)
  end

  def increment_place(place)
    if @digits[place] == 9
      increment_place(place - 1)
      @digits[place] = @digits[place - 1]
    else
      @digits[place] += 1
    end
  end

  def after_end?
    place = 0
    while place <= @last_place
      return false if @digits[place] < @digits_end[place]
      place += 1
    end
    true
  end

  def count_matches
    matches = 0
    while !after_end?
      passes = (@require_one_double ? passes_tests_require_one_double(@digits) : passes_tests(@digits))
      log.debug "passes? #{@digits.join('')} #{passes}"
      matches += 1 if passes
      increment
    end
    matches
  end
end

def num_to_digits(num)
  num.to_s.chars.map(&:to_i)
end

class Counter
  def initialize(num_start, num_end)
    @digits = num_to_digits(num_start)
  end
end

def main
  input = [264360, 746325]

  log.info "Part 1:"
  #log.info measure {part1_try1(input)}
  #log.info measure {part1_try2(input)}
  #log.info measure {part1_try3(input)}

  log.info "Part 2:"
  log.info measure {part2(input)}
end

if __FILE__ == $0
  main
end