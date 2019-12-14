require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [".#..#\n.....\n#####\n....#\n...##", [7, 7, 6, 7, 7, 7, 5, 7, 8, 7], [3, 4], 8],
    ["......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####", nil, [5, 8], 33],
    ["#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.", nil, [1, 2], 35],
    [".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..", nil, [6, 3], 41],
    [".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##", nil, [11, 13], 210],
  ]

  def test_examples
    EXAMPLES.each do |input_str, expected_counts, expected_best_location, expected_best_count|
      asteroids = process_input(input_str)
      counts = counts_per_asteroid(asteroids)
      if expected_counts
        assert_equal(expected_counts, asteroids.map {|a| counts[a]})
      end
      location, count = best_location(counts)
      assert_equal(expected_best_location, location)
      assert_equal(expected_best_count, count)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    asteroids = process_input(input_str)
    counts = counts_per_asteroid(asteroids)
    location, count = best_location(counts)
    assert_equal([13, 17], location)
    assert_equal(269, count)
  end
end

# class TestPart2 < Minitest::Test
#   EXAMPLES = [
#   ]

#   def test_examples
#     EXAMPLES.each do |input_str, expected_output|
#       input = process_input(input_str)
#       res = part2(input)
#       assert_equal(expected_output, res)
#     end
#   end

#   def test_input
#     input_str = File.read(INPUT_FILE)
#     input = process_input(input_str)
#     res = part2(input)
#     assert_equal(nil, res)
#   end
# end
