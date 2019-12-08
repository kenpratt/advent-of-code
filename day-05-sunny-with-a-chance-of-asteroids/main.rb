require_relative '../utils/log'
require_relative '../utils/profile'

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  raw_input.strip.split(',').map(&:to_i)
end

def modify_input(input, noun, verb)
  out = input.clone
  out[1] = noun
  out[2] = verb
  out  
end

def part1(input)
  arr = input.clone
  i = 0
  while i < arr.size
    op = arr[i]
    case op
    when 1, 2
      val1 = arr[arr[i + 1]]
      val2 = arr[arr[i + 2]]
      res = op == 1 ? val1 + val2 : val1 * val2
      arr[arr[i + 3]] = res
      i += 4
    when 2
      i += 4
    when 99
      return arr
    else
      raise "Unknown opcode: #{op}"
    end
  end
  raise "Didn't halt"
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