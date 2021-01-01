require 'minitest/autorun'

require_relative './main'
require_relative 'port_monitor'

log.level = Logger::INFO

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_input1
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)

    #  What is the Y value of the first packet sent to address 255?
    monitor = PortMonitor.new(255) {|log| log.any?}

    res = run_network(program, 50, monitor)
    assert_equal([[20771, 14834]], monitor.log)
  end
end

class TestPart2 < Minitest::Test
  def test_input2
    input_str = File.read(INPUT_FILE)
    program = parse_program(input_str)

    # Monitor packets released to the computer at address 0 by the NAT.
    monitor = PortMonitor.new(0) do |log|
      # What is the first Y value delivered by the NAT to the computer at address 0 twice in a row?
      log.size >= 2 && log[-2][1] == log[-1][1]
    end

    res = run_network(program, 50, monitor)
    assert_equal(10215, monitor.log[-1][1])
  end
end
