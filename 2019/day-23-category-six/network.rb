require_relative 'computer'

class Network
  attr_reader :buffers

  def self.run(program, size)
    network = self.new(program, size)
    network.run
    network
  end

  def initialize(program, size)
    @nics = size.times.map {|a| [a, IntcodeComputer.new(program, [a], "NIC-#{a}")]}.to_h
    @buffers = Hash.new {|h, k| h[k] = []}
  end

  def run
    while @buffers[255].empty?
      tick
    end
  end

  def tick
    @nics.each do |address, nic|
      nic.tick

      if nic.blocked?
        input = receive_packet(address)
        nic.add_input_arr(input)
      end

      if nic.output.size == 3
        output = nic.clear_output
        send_packet(*output)
      end
    end
  end

  def send_packet(address, x, y)
    @buffers[address] << [x, y]
  end

  def receive_packet(address)
    if @buffers[address].any?
      @buffers[address].shift
    else
      [-1]
    end
  end
end
