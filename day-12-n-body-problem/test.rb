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
        pot:  6; kin:  6; total: 36
        pot:  9; kin:  5; total: 45
        pot: 10; kin:  8; total: 80
        pot:  6; kin:  3; total: 18
      EOM
      179,
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
        pot: 29; kin: 10; total:290
        pot: 32; kin: 19; total:608
        pot: 41; kin: 14; total:574
        pot: 52; kin:  9; total:468      
      EOM
      1940,
    ],
  ]

  def test_examples
    EXAMPLES.each do |input_str, iterations, expected_states, expected_energy, expected_total_energy|
      input = process_input(input_str)
      simulation = simulate(input, iterations)
      assert_equal(expected_states.strip, simulation.states_to_s)
      assert_equal(expected_energy.strip, simulation.energy_to_s)
      assert_equal(expected_total_energy, simulation.total_energy)
    end
  end

  def test_input
    input_str = File.read(INPUT_FILE)
    input = process_input(input_str)
    simulation = simulate(input, 1000)
    assert_equal(14780, simulation.total_energy)
  end
end

class TestPart2 < Minitest::Test
  EXAMPLES = [
    # [
    #   <<~EOM ,
    #     <x=-1, y=0, z=2>
    #     <x=2, y=-10, z=-7>
    #     <x=4, y=-8, z=8>
    #     <x=3, y=5, z=-1>
    #   EOM
    #   2772,
    # ],
    [
      <<~EOM ,
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
      EOM
      4686774924,
    ],
  ]

  def test_examples
    EXAMPLES.each do |input_str, expected_num_steps|
      log.debug "running test #{input_str.inspect} #{expected_num_steps}"
      input = process_input(input_str)
      steps = 2_000_000
      measure {simulate(input, steps)}
      profile {simulate(input, steps)}
      # steps = profile {simulate_until_repeat(input, 200000)}
      # assert_equal(expected_num_steps, steps)
    end
  end

  # def test_input
  #   input_str = File.read(INPUT_FILE)
  #   input = process_input(input_str)
  #   steps = simulate_until_repeat(input)
  #   assert_equal(0, steps)
  # end
end
