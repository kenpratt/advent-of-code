require "minitest/autorun"

INPUT_FILE = File.join(__dir__, 'input.txt')

def run_program(input)
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

class TestExamples < Minitest::Test
  TESTS = [
    ['1,9,10,3,2,3,11,0,99,30,40,50', '3500,9,10,70,2,3,11,0,99,30,40,50'],
    ['1,0,0,0,99', '2,0,0,0,99'],
    ['2,3,0,3,99', '2,3,0,6,99'],
    ['2,4,4,5,99,0', '2,4,4,5,99,9801'],
    ['1,1,1,4,99,5,6,0,99', '30,1,1,4,2,5,6,0,99'],
  ]

  def test_examples
    TESTS.each do |input_str, expected_output|
      res = run_program(input_str.split(',').map(&:to_i))
      assert_equal(expected_output, res.join(','))
    end
  end
end

def modify_input(input, noun, verb)
  out = input.clone
  out[1] = noun
  out[2] = verb
  out  
end

def main
  input = File.readlines(INPUT_FILE)[0].split(',').map(&:to_i)
  run_program(input)

  puts "Part 1:"
  # replace position 1 with the value 12
  # replace position 2 with the value 2
  modified_input = modify_input(input, 12, 2)
  puts run_program(modified_input)[0]

  puts "Part 2:"
  combinations = (0..99).map {|x| (0..99).map {|y| [x, y]}}.flatten(1)
  result = combinations.find do |noun, verb|
    run_program(modify_input(input, noun, verb))[0] == 19690720
  end
  puts result[0] * 100 + result[1]
end

main