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
  def test_input2
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    input = input * 10000
    result_offset = input[0, 7].join('').to_i

    output = nil
    # cache_patterns(input.size)
    # log.debug 'cached patterns'
    1.times do |i|
      log.debug "run #{i}"
      output = profile{run_phase(input, 10)}
      input = output
    end

    result = output[result_offset, 8]
    binding.pry
    assert_equal('00000000', result.join(''))
  end
end
