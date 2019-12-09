require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  nil
end

def part1(input)
  nil
end

def part2(input)
  nil
end

def main
  raw_input = File.read(INPUT_FILE)
  input = process_input(raw_input)

  log.info "Part 1:"
  log.info measure{part1(input)}

  log.info "Part 2:"
  log.info measure{part2(input)}
end

if __FILE__ == $0
  main
end