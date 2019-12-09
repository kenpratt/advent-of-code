require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestDay2 < Minitest::Test
  EXAMPLES = [
    ['1,9,10,3,2,3,11,0,99,30,40,50', '3500,9,10,70,2,3,11,0,99,30,40,50'],
    ['1,0,0,0,99', '2,0,0,0,99'],
    ['2,3,0,3,99', '2,3,0,6,99'],
    ['2,4,4,5,99,0', '2,4,4,5,99,9801'],
    ['1,1,1,4,99,5,6,0,99', '30,1,1,4,2,5,6,0,99'],
    ['1101,100,-1,4,0', '1101,100,-1,4,99'],
  ]

  INPUT_FILE = File.join(__dir__, '..', 'day-02-program-alarm', 'input.txt')

  def test_examples
    EXAMPLES.each do |program_str, expected_result|
      program = parse_program(program_str)
      computer = run_program(program, [])
      result = computer.memory.join(',')
      assert_equal(result, expected_result)
    end
  end

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)

    # replace position 1 with the value 12
    # replace position 2 with the value 2
    modified_program = modify_program(program, 12, 2)
    computer = run_program(modified_program, [])
    result = computer.memory[0]
    assert_equal(result, 3166704)
  end

  def test_part_2
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)

    combinations = (0..99).map {|x| (0..99).map {|y| [x, y]}}.flatten(1)
    winner = combinations.find do |noun, verb|
      modified_program = modify_program(program, noun, verb)
      computer = run_program(modified_program, [])
      result = computer.memory[0]
      result == 19690720
    end
    assert_equal(winner, [80, 18])
  end
end

class TestDay5 < Minitest::Test
  EXAMPLES = [
    ['3,9,8,9,10,9,4,9,99,-1,8', 8, 1], # 8 == 8
    ['3,9,8,9,10,9,4,9,99,-1,8', 7, 0], # 7 == 8
    ['3,9,7,9,10,9,4,9,99,-1,8', 7, 1], # 7 < 8
    ['3,9,7,9,10,9,4,9,99,-1,8', 8, 0], # 8 < 8
    ['3,9,7,9,10,9,4,9,99,-1,8', 9, 0], # 9 < 8
    ['3,3,1108,-1,8,3,4,3,99', 8, 1], # 8 == 8
    ['3,3,1108,-1,8,3,4,3,99', 7, 0], # 7 == 8
    ['3,3,1107,-1,8,3,4,3,99', 7, 1], # 7 < 8
    ['3,3,1107,-1,8,3,4,3,99', 8, 0], # 8 < 8
    ['3,3,1107,-1,8,3,4,3,99', 9, 0], # 9 < 8
    ['3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9', 0, 0],
    ['3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9', 1, 1],
    ['3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9', 2, 1],
    ['3,3,1105,-1,9,1101,0,0,12,4,12,99,1', 0, 0],
    ['3,3,1105,-1,9,1101,0,0,12,4,12,99,1', 1, 1],
    ['3,3,1105,-1,9,1101,0,0,12,4,12,99,1', 2, 1],
    ['3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99', 7, 999],
    ['3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99', 8, 1000],
    ['3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99', 9, 1001],
  ]

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    computer = run_program(program, [1])
    result = computer.output.last
    assert_equal(result, 6745903)
  end

  def test_examples
    EXAMPLES.each do |program_str, input, expected_result|
      program = parse_program(program_str)
      computer = run_program(program, [input])
      result = computer.output.last
      assert_equal(expected_result, result)
    end
  end

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    computer = run_program(program, [5])
    result = computer.output.last
    assert_equal(result, 9168267)
  end
end
