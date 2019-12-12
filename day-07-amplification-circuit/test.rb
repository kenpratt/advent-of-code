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
      assert_equal(expected_result, result)
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
    assert_equal(3166704, result)
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
    assert_equal([80, 18], winner)
  end
end

class TestDay5 < Minitest::Test
  INPUT_FILE = File.join(__dir__, '..', 'day-05-sunny-with-a-chance-of-asteroids', 'input.txt')

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
    assert_equal(6745903, result)
  end

  def test_examples
    EXAMPLES.each do |program_str, input, expected_result|
      program = parse_program(program_str)
      computer = run_program(program, [input])
      result = computer.output.last
      assert_equal(expected_result, result)
    end
  end

  def test_part_2
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    computer = run_program(program, [5])
    result = computer.output.last
    assert_equal(9168267, result)
  end
end

class TestDay7 < Minitest::Test
  PART1_EXAMPLES = [
    ['3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0', '4,3,2,1,0', 43210],
    ['3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0', '0,1,2,3,4', 54321],
    ['3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0', '1,0,4,3,2', 65210],
  ]

  PART2_EXAMPLES = [
    ['3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5', '9,8,7,6,5', 139629729],
    ['3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10', '9,7,8,5,6', '18216']
  ]

  def test_thruster_signal
    PART1_EXAMPLES.each do |program_str, phase_settings_str, expected_thruster_signal|
      program = parse_program(program_str)
      phase_settings = phase_settings_str.split(',').map(&:to_i)
      thruster_signal = run_amplifiers(program, phase_settings, 0)
      assert_equal(expected_thruster_signal, thruster_signal)
    end
  end

  def test_best_phase_setting
    PART1_EXAMPLES.each do |program_str, expected_phase_settings_str, expected_thruster_signal|
      program = parse_program(program_str)
      expected_phase_settings = expected_phase_settings_str.split(',').map(&:to_i)
      thruster_signal, best_phase_settings = *find_best_phase_setting_permutation(program, (0..4).to_a, 0)
      assert_equal(expected_thruster_signal, thruster_signal)
      assert_equal(expected_phase_settings, best_phase_settings)
    end
  end

  def test_part_1
    program_str = File.read(INPUT_FILE)
    program = parse_program(program_str)
    thruster_signal, best_phase_settings = *find_best_phase_setting_permutation(program, (0..4).to_a, 0)
    assert_equal(19650, thruster_signal)
    assert_equal([2, 0, 1, 4, 3], best_phase_settings)
  end
end
