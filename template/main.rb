require 'minitest/autorun'
require 'logger'

$log = Logger.new(STDOUT)
def log; $log; end

$log.level = Logger::DEBUG
#$log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

def process_input(raw_input)
  nil
end

def solution(input)
  nil
end
  
def main
  raw_input = File.read(INPUT_FILE)
  # ...
  input = process_input(raw_input)

  log.info "Part 1:"
  log.info solution(input)

  log.info "Part 2:"
  log.info solution(input)
end

class TestExamples < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = solution(input)
      assert_equal output, res
    end
  end
end

main