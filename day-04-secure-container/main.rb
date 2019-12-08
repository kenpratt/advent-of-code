require_relative '../utils/log'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  nil
end

def part1(input)
  num_start, num_end = *input
  num_start.upto(num_end).count {|x| passes_tests(x)}
end

def passes_tests(num)
  digits = num_to_digits(num)
  found_repeat = false
  last = digits[0]
  digits[1..-1].each do |curr|
    return false if curr < last
    found_repeat = true if curr == last
    last = curr
  end
  found_repeat
end

def num_to_digits(num)
  num.to_s.chars.map(&:to_i)
end

def part1_(input)
  num_start, num_end = *input
  Counter.new(num_start, num_end)
end

class Counter
  def initialize(num_start, num_end)
    @digits = num_to_digits(num_start)
  end
end

def part2(input)
  nil
end

def main
  if ARGV[0] == 'debug'
    log.level = Logger::DEBUG
  end

  input = [264360, 746325]

  log.info "Part 1:"
  log.info part1(input)

  #log.info "Part 2:"
  #log.info part2(input)
end

if __FILE__ == $0
  main
end