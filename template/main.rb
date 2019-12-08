require_relative '../utils/log'

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
  if ARGV[0] == 'debug'
    log.level = Logger::DEBUG
  end

  raw_input = File.read(INPUT_FILE)
  input = process_input(raw_input)

  log.info "Part 1:"
  log.info part1(input)

  log.info "Part 2:"
  log.info part2(input)
end

if __FILE__ == $0
  main
end