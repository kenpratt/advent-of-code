require 'timeout'
require 'io/console'

require_relative '../utils/log'

def read_char_from_keyboard(timeout)
  STDIN.echo = false
  
  char = nil
  begin
    Timeout::timeout(timeout) do
      char = STDIN.getch
      log.debug "read char: #{char}"
    end
  rescue Timeout::Error
    log.debug "timeout"
  end
  char
end

def timestamp
  Process.clock_gettime(Process::CLOCK_MONOTONIC)
end

def elapsed_since_timestamp(since)
  now = timestamp
  elapsed = now - since
  [elapsed, now]
end