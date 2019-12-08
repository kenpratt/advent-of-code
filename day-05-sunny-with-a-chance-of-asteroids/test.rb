require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestOldTests < Minitest::Test
  TESTS = [
    ['1,9,10,3,2,3,11,0,99,30,40,50', '3500,9,10,70,2,3,11,0,99,30,40,50'],
    ['1,0,0,0,99', '2,0,0,0,99'],
    ['2,3,0,3,99', '2,3,0,6,99'],
    ['2,4,4,5,99,0', '2,4,4,5,99,9801'],
    ['1,1,1,4,99,5,6,0,99', '30,1,1,4,2,5,6,0,99'],
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = part1(input)
      assert_equal(output, res.join(','))
    end
  end
end  

class TestPart1 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = part1(input)
      assert_equal(output, res.join(','))
    end
  end
end

class TestPart2 < Minitest::Test
  TESTS = [
  ]

  def test_examples
    TESTS.each do |raw_input, output|
      input = process_input(raw_input)
      res = part2(input)
      assert_equal(output, res.join(','))
    end
  end
end
