require 'ruby-prof'
require_relative './log'

def profile
  output = nil

  profile_result = RubyProf.profile do
    output = yield
  end
  
  # print a graph profile to text
  printer = RubyProf::GraphPrinter.new(profile_result)
  printer.print(STDOUT, {})

  output
end

def measure
  return profile if ARGV[0] == 'profile'

  starting = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  output = yield
  ending = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  took = ending - starting
  log.info "took #{took}"
  output
end