class PortMonitor
  attr_reader :log

  def initialize(port, &complete)
    @port = port
    @complete = complete
    @log = []
  end

  def trace(address, message)
    if @port == address
      @log << message
    end
  end

  def continue?
    !@complete.call(@log)
  end
end