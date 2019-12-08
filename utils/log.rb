require 'logger'

$log = Logger.new(STDOUT)
def log; $log; end

log.level = Logger::INFO
