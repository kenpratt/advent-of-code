require 'logger'

$log = Logger.new(STDOUT)
def log; $log; end

if ARGV[0] == 'debug'
  log.level = Logger::DEBUG
else
  log.level = Logger::INFO
end  
