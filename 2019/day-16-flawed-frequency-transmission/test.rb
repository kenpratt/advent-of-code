require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  PER_PHASE_EXAMPLES = [
    ['12345678', ['48226158', '34040438', '03415518', '01029498']],
  ]

  FINAL_OUTPUT_EXAMPLES = [
    ['80871224585914546619083218645595', 100, '24176176'],
    ['19617804207202209144916044189917', 100, '73745418'],
    ['69317163492948606335995924319873', 100, '52432133'],
  ]

  def test_per_phase_examples
    PER_PHASE_EXAMPLES.each do |input_str, expected_phase_outputs|
      input = process_input(input_str)
      expected_phase_outputs.each do |expected_output|
        output = run_phase(input)
        assert_equal(process_input(expected_output), output)
        input = output
      end
    end
  end

  def test_final_output_examples
    FINAL_OUTPUT_EXAMPLES.each do |input_str, num_phases, expected_output_prefix|
      input = process_input(input_str)
      output = nil
      num_phases.times do
        output = run_phase(input)
        input = output
      end
      assert_equal(
        process_input(expected_output_prefix),
        output[0, expected_output_prefix.size],
      )
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    output = nil
    100.times do |i|
      log.debug "run #{i}"
      output = run_phase(input)
      input = output
    end
    assert_equal('50053207', output[0, 8].join(''))
  end
end

class TestPart2 < Minitest::Test
  FINAL_OUTPUT_EXAMPLES = [
    ['03036732577212944063491565474664', 10000, 100, '84462026'],
    ['02935109699940807407585447034323', 10000, 100, '78725270'],
    ['03081770884921959731165446850517', 10000, 100, '53553731'],
  ]

  def test_final_output_examples2
    FINAL_OUTPUT_EXAMPLES.each do |input_str, input_repeat, num_phases, expected_output_prefix|
      input = process_input(input_str)
      result = calculate_result_with_offset_output(
        input,
        input_repeat,
        num_phases,
        expected_output_prefix.size,
      )
      assert_equal(expected_output_prefix, result.join(''))
    end
  end

  def test_input2
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    result = calculate_result_with_offset_output(
      input,
      10000,
      100,
      8,
    )    
    assert_equal('32749588', result.join(''))
  end
end
