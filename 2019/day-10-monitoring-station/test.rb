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
      asteroids, _ = process_input(input_str)
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
    asteroids, _ = process_input(input_str)
    counts = counts_per_asteroid(asteroids)
    location, count = best_location(counts)
    assert_equal([13, 17], location)
    assert_equal(269, count)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    [
      ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##",
      {0=>[8, 1], 1=>[9, 0], 2=>[9, 1], 3=>[10, 0], 4=>[9, 2], 5=>[11, 1], 6=>[12, 1], 7=>[11, 2], 8=>[15, 1], 9=>[12, 2], 10=>[13, 2], 11=>[14, 2], 12=>[15, 2], 13=>[12, 3], 14=>[16, 4], 15=>[15, 4], 16=>[10, 4], 17=>[4, 4], 18=>[2, 4], 19=>[2, 3], 20=>[0, 2], 21=>[1, 2], 22=>[0, 1], 23=>[1, 1], 24=>[5, 2], 25=>[1, 0], 26=>[5, 1], 27=>[6, 1], 28=>[6, 0], 29=>[7, 0], 30=>[8, 0], 31=>[10, 1], 32=>[14, 0], 33=>[16, 1], 34=>[13, 3], 35=>[14, 3]},
    ],
    [
      ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.####X#####...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##",
      {0=>[11, 12], 1=>[12, 1], 2=>[12, 2], 9=>[12, 8], 19=>[16, 0], 49=>[16, 9], 99=>[10, 16], 198=>[9, 6], 199=>[8, 2], 200=>[10, 9], 298=>[11, 1]},
    ],
  ]

  def test_examples
    EXAMPLES.each do |input_str, expected_vaporizations|
      asteroids, starting_position = process_input(input_str)
      vaporized_order = vaporize_asteroids(starting_position, asteroids)
      assert_equal(asteroids.size, vaporized_order.size)
      expected_vaporizations.each do |index, asteroid|
        assert_equal(asteroid, vaporized_order[index], "Index: #{index}")
      end
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    asteroids, starting_position = process_input(input_str)
    assert_nil(starting_position)
    vaporized_order = vaporize_asteroids([13, 17], asteroids)
    assert_equal(asteroids.size, vaporized_order.size)

    asteroid = vaporized_order[199]
    assert_equal([6, 12], asteroid)
  end
end
