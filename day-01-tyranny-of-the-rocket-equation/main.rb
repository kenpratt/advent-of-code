require "minitest/autorun"

INPUT_FILE = File.join(__dir__, 'input.txt')

# Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
#
# For example:
#
# For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
# For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
# For a mass of 1969, the fuel required is 654.
# For a mass of 100756, the fuel required is 33583.
def base_fuel_required(mass)
  (mass / 3) - 2
end

# So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:
# A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
# At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
# The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
def full_fuel_required(mass)
  res = base_fuel_required(mass)
  if res > 0
    res + full_fuel_required(res)
  else
    0
  end
end

class TestFeulRequired < Minitest::Test
  def setup
  end

  def test_base_fuel_required
    assert_equal 2, base_fuel_required(12)
    assert_equal 2, base_fuel_required(14)
    assert_equal 654, base_fuel_required(1969)
    assert_equal 33583, base_fuel_required(100756)
  end

  def test_full_fuel_required
    assert_equal 2, full_fuel_required(14)
    assert_equal 966, full_fuel_required(1969)
    assert_equal 50346, full_fuel_required(100756)
  end  
end

def main
  modules = File.readlines(INPUT_FILE).map {|l| l.strip.to_i}

  puts "Part 1:"
  puts modules.map {|m| base_fuel_required(m)}.sum

  puts "Part 2:"
  puts modules.map {|m| full_fuel_required(m)}.sum
end

main