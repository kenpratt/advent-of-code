require_relative 'computer'

class Network
  attr_reader :buffers

  def self.run(program, size, port_monitor)
    network = self.new(program, size, port_monitor)
    network.run
    network
  end

  def initialize(program, size, port_monitor)
    @nics = size.times.map {|a| [a, IntcodeComputer.new(program, [a], "NIC-#{a}")]}.to_h
    @port_monitor = port_monitor
    @buffers = Hash.new {|h, k| h[k] = []}
    @idle = Hash.new(false)
  end

  def run()
    while @port_monitor.continue?
      tick
    end
  end

  def tick
    @nics.each do |address, nic|
      nic.tick

      if nic.blocked?
        input = receive_packet(address)
        nic.add_input_arr(input)
        @idle[address] = (input == [-1])
      end

      if nic.output.size == 3
        output = nic.clear_output
        send_packet(*output)
        @idle[address] = false
      end
    end

    # Once the network is idle, the NAT sends only the last packet it received to address 0
    if network_idle? && @buffers[255].any?
      input = @buffers[255].last
      @buffers[255].clear
      raise "Bad packet on NAT interface: #{input.inspect}" if input.size != 2
      send_packet(0, *input)
    end
  end

  def send_packet(address, x, y)
    @port_monitor.trace(address, [x, y])
    @buffers[address] << [x, y]
  end

  def receive_packet(address)
    if @buffers[address].any?
      @buffers[address].shift
    else
      [-1]
    end
  end

  def network_idle?
    @nics.all? {|address, _| @idle[address]}
  end
end
