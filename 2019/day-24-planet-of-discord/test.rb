  require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

INPUT_FILE = File.join(__dir__, 'input.txt')

class TestPart1 < Minitest::Test
  def test_example1_basics
    input_str = "....#\n#..#.\n#..##\n..#..\n#...."
    min1_output_str = "#..#.\n####.\n###.#\n##.##\n.##.."
    min2_output_str = "#####\n....#\n....#\n...#.\n#.###"
    min3_output_str = "#....\n####.\n...##\n#.##.\n.##.#"
    min4_output_str = "####.\n....#\n##..#\n.....\n##..."

    simulation = Simulation.new(input_str, false)
    assert_equal(input_str, simulation.to_s.strip)
    simulation.tick
    assert_equal(min1_output_str, simulation.to_s.strip)
    simulation.tick
    assert_equal(min2_output_str, simulation.to_s.strip)
    simulation.tick
    assert_equal(min3_output_str, simulation.to_s.strip)
    simulation.tick
    assert_equal(min4_output_str, simulation.to_s.strip)
  end

  def test_example1_until_repeat
    input_str = "....#\n#..#.\n#..##\n..#..\n#...."
    output_str = ".....\n.....\n.....\n#....\n.#..."

    simulation = Simulation.new(input_str, false)
    simulation.run_until_repeat

    assert_equal(output_str, simulation.to_s.strip)
    assert_equal(2129920, simulation.biodiversity_rating)
  end

  def test_input1
    input_str = File.read(INPUT_FILE)

    simulation = Simulation.new(input_str, false)
    simulation.run_until_repeat

    assert_equal(32776479, simulation.biodiversity_rating)
  end
end

class TestPart2 < Minitest::Test
  def test_example2
    input_str = "....#\n#..#.\n#..##\n..#..\n#...."
    min10_output_str = <<~EOF
      Depth -5:
      ..#..
      .#.#.
      ..?.#
      .#.#.
      ..#..

      Depth -4:
      ...#.
      ...##
      ..?..
      ...##
      ...#.

      Depth -3:
      #.#..
      .#...
      ..?..
      .#...
      #.#..

      Depth -2:
      .#.##
      ....#
      ..?.#
      ...##
      .###.

      Depth -1:
      #..##
      ...##
      ..?..
      ...#.
      .####

      Depth 0:
      .#...
      .#.##
      .#?..
      .....
      .....

      Depth 1:
      .##..
      #..##
      ..?.#
      ##.##
      #####

      Depth 2:
      ###..
      ##.#.
      #.?..
      .#.##
      #.#..

      Depth 3:
      ..###
      .....
      #.?..
      #....
      #...#

      Depth 4:
      .###.
      #..#.
      #.?..
      ##.#.
      .....

      Depth 5:
      ####.
      #..#.
      #.?#.
      ####.
      .....
    EOF

    simulation = Simulation.new(input_str, true)
    simulation.run(10)
    assert_equal(min10_output_str.strip, simulation.to_s.strip)
    assert_equal(99, simulation.count_bugs)
  end

  def test_input2
    input_str = File.read(INPUT_FILE)

    simulation = Simulation.new(input_str, true)
    simulation.run(200)
    assert_equal(2017, simulation.count_bugs)
  end
end
