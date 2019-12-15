require 'minitest/autorun'

require_relative './main'

log.level = Logger::DEBUG

class TestPart1 < Minitest::Test
  EXAMPLES = [
    [
      <<~EOM ,
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
      EOM
      10,
      <<~EOM ,
        pos=<x=  2, y=  1, z= -3>, vel=<x= -3, y= -2, z=  1>
        pos=<x=  1, y= -8, z=  0>, vel=<x= -1, y=  1, z=  3>
        pos=<x=  3, y= -6, z=  1>, vel=<x=  3, y=  2, z= -3>
        pos=<x=  2, y=  0, z=  4>, vel=<x=  1, y= -1, z= -1>
      EOM
      <<~EOM ,
        pot: 2 + 1 + 3 =  6;   kin: 3 + 2 + 1 = 6;   total:  6 * 6 = 36
        pot: 1 + 8 + 0 =  9;   kin: 1 + 1 + 3 = 5;   total:  9 * 5 = 45
        pot: 3 + 6 + 1 = 10;   kin: 3 + 2 + 3 = 8;   total: 10 * 8 = 80
        pot: 2 + 0 + 4 =  6;   kin: 1 + 1 + 1 = 3;   total:  6 * 3 = 18
      EOM
    ],
    [
      <<~EOM ,
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
      EOM
      100,
      <<~EOM ,
        pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
        pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
        pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
        pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>
      EOM
      <<~EOM ,
        pot:  8 + 12 +  9 = 29;   kin: 7 +  3 + 0 = 10;   total: 29 * 10 = 290
        pot: 13 + 16 +  3 = 32;   kin: 3 + 11 + 5 = 19;   total: 32 * 19 = 608
        pot: 29 + 11 +  1 = 41;   kin: 3 +  7 + 4 = 14;   total: 41 * 14 = 574
        pot: 16 + 13 + 23 = 52;   kin: 7 +  1 + 1 =  9;   total: 52 *  9 = 468      
      EOM
    ],
  ]

  def test_examples
    EXAMPLES.each do |input_str, iterations, expected_states, expected_energy|
      input = process_input(input_str)
      simulation = simulate(input, iterations)
      assert_equal(expected_states.strip, simulation.states_to_s)
      #assert_equal(expected_energy.strip, simulation.energy_to_s)
    end
  end

  # def test_input
  #   input_str = File.read(INPUT_FILE)
  #   input = process_input(input_str)
  #   res = part1(input)
  #   assert_equal(nil, res)
  # end
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
