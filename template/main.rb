require "minitest/autorun"

INPUT_FILE = File.join(__dir__, 'input.txt')

def run(input)
  nil
end

def process_input(input)
end
  
def main
  lines = File.readlines(INPUT_FILE)
  # ...
  input = process_input(lines)

  puts "Part 1:"
  puts run(input)

  puts "Part 2:"
  puts run(input)
end

class TestExamples < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |input, output|
      res = run_program(process_input(input))
      assert_equal output, res
    end
  end
end

main