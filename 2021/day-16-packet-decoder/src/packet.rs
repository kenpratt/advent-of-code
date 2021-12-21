use crate::bitstream::BitStream;

#[derive(Debug, PartialEq)]
pub enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    pub fn read(stream: &mut BitStream) -> Packet {
        let version = stream.read(3);
        let type_id = stream.read(3);
        match type_id {
            4 => Self::read_literal(stream, version),
            _ => Self::read_operator(stream, version, type_id),
        }
    }

    fn read_literal(stream: &mut BitStream, version: usize) -> Packet {
        let mut value = 0;
        loop {
            let last_group = stream.read(1);
            let nibble = stream.read(4);
            value = (value << 4) | nibble;
            if last_group == 0 {
                break;
            }
        }
        Self::Literal { version, value }
    }

    fn read_operator(stream: &mut BitStream, version: usize, type_id: usize) -> Packet {
        let sub_packets = Self::read_operator_sub_packets(stream);
        Self::Operator {
            version,
            type_id,
            sub_packets,
        }
    }

    fn read_operator_sub_packets(stream: &mut BitStream) -> Vec<Packet> {
        let length_type_id = stream.read(1);
        match length_type_id {
            0 => {
                let bits_to_read = stream.read(15);
                let target_stream_bits_read = stream.bits_read() + bits_to_read;
                let mut packets = vec![];
                while stream.bits_read() < target_stream_bits_read {
                    let packet = Self::read(stream);
                    packets.push(packet);
                }
                packets
            }
            1 => {
                let packets_to_read = stream.read(11);
                (0..packets_to_read).map(|_| Self::read(stream)).collect()
            }
            _ => panic!("Unexpected length type id: {}", length_type_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "D2FE28";
    static EXAMPLE2: &str = "38006F45291200";
    static EXAMPLE3: &str = "EE00D40C823060";
    static EXAMPLE4: &str = "8A004A801A8002F478";
    static EXAMPLE5: &str = "620080001611562C8802118E34";
    static EXAMPLE6: &str = "C0015000016115A2E0802F182340";
    static EXAMPLE7: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_packet_read_example1() {
        let mut stream = BitStream::from_str(EXAMPLE1);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Literal {
                version: 6,
                value: 2021,
            },
        );
    }

    #[test]
    fn test_packet_read_example2() {
        let mut stream = BitStream::from_str(EXAMPLE2);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 1,
                type_id: 6,
                sub_packets: vec![
                    Packet::Literal {
                        version: 6,
                        value: 10,
                    },
                    Packet::Literal {
                        version: 2,
                        value: 20,
                    },
                ],
            },
        );
    }

    #[test]
    fn test_packet_read_example3() {
        let mut stream = BitStream::from_str(EXAMPLE3);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 7,
                type_id: 3,
                sub_packets: vec![
                    Packet::Literal {
                        version: 2,
                        value: 1,
                    },
                    Packet::Literal {
                        version: 4,
                        value: 2,
                    },
                    Packet::Literal {
                        version: 1,
                        value: 3,
                    },
                ],
            },
        );
    }

    #[test]
    fn test_packet_read_example4() {
        let mut stream = BitStream::from_str(EXAMPLE4);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 4,
                type_id: 2,
                sub_packets: vec![Packet::Operator {
                    version: 1,
                    type_id: 2,
                    sub_packets: vec![Packet::Operator {
                        version: 5,
                        type_id: 2,
                        sub_packets: vec![Packet::Literal {
                            version: 6,
                            value: 15,
                        }],
                    }],
                }],
            },
        );
    }

    #[test]
    fn test_packet_read_example5() {
        let mut stream = BitStream::from_str(EXAMPLE5);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 3,
                type_id: 0,
                sub_packets: vec![
                    Packet::Operator {
                        version: 0,
                        type_id: 0,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 0,
                                value: 10,
                            },
                            Packet::Literal {
                                version: 5,
                                value: 11,
                            },
                        ],
                    },
                    Packet::Operator {
                        version: 1,
                        type_id: 0,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 0,
                                value: 12,
                            },
                            Packet::Literal {
                                version: 3,
                                value: 13,
                            },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn test_packet_read_example6() {
        let mut stream = BitStream::from_str(EXAMPLE6);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 6,
                type_id: 0,
                sub_packets: vec![
                    Packet::Operator {
                        version: 0,
                        type_id: 0,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 0,
                                value: 10,
                            },
                            Packet::Literal {
                                version: 6,
                                value: 11,
                            },
                        ],
                    },
                    Packet::Operator {
                        version: 4,
                        type_id: 0,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 7,
                                value: 12,
                            },
                            Packet::Literal {
                                version: 0,
                                value: 13,
                            },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn test_packet_read_example7() {
        let mut stream = BitStream::from_str(EXAMPLE7);
        let packet = Packet::read(&mut stream);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 5,
                type_id: 0,
                sub_packets: vec![Packet::Operator {
                    version: 1,
                    type_id: 0,
                    sub_packets: vec![Packet::Operator {
                        version: 3,
                        type_id: 0,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 7,
                                value: 6,
                            },
                            Packet::Literal {
                                version: 6,
                                value: 6,
                            },
                            Packet::Literal {
                                version: 5,
                                value: 12,
                            },
                            Packet::Literal {
                                version: 2,
                                value: 15,
                            },
                            Packet::Literal {
                                version: 2,
                                value: 15,
                            },
                        ],
                    }],
                }],
            },
        );
    }
}
