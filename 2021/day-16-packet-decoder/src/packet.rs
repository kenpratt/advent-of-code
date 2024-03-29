use crate::bitstream::BitStream;

#[derive(Debug, PartialEq)]
pub enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operation {
        version: usize,
        operator: Operator,
        operands: Vec<Packet>,
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
        let operator = Operator::from_type_id(type_id);
        let operands = Self::read_sub_packets(stream);
        Self::Operation {
            version,
            operator,
            operands,
        }
    }

    fn read_sub_packets(stream: &mut BitStream) -> Vec<Packet> {
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

    pub fn value(&self) -> usize {
        match self {
            Self::Literal { version: _, value } => *value,
            Self::Operation {
                version: _,
                operator,
                operands,
            } => {
                let values: Vec<usize> = operands.iter().map(|p| p.value()).collect();
                operator.apply(&values)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn from_type_id(type_id: usize) -> Operator {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!("Unrecognized operation type id: {}", type_id),
        }
    }

    fn apply(&self, values: &[usize]) -> usize {
        match self {
            Self::Sum => values.iter().sum(),
            Self::Product => values.iter().fold(1, |acc, v| acc * v),
            Self::Minimum => *values.iter().min().unwrap(),
            Self::Maximum => *values.iter().max().unwrap(),
            Self::GreaterThan => {
                assert_eq!(values.len(), 2);
                (values[0] > values[1]).into()
            }
            Self::LessThan => {
                assert_eq!(values.len(), 2);
                (values[0] < values[1]).into()
            }
            Self::EqualTo => {
                assert_eq!(values.len(), 2);
                (values[0] == values[1]).into()
            }
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
            Packet::Operation {
                version: 1,
                operator: Operator::LessThan,
                operands: vec![
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
            Packet::Operation {
                version: 7,
                operator: Operator::Maximum,
                operands: vec![
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
            Packet::Operation {
                version: 4,
                operator: Operator::Minimum,
                operands: vec![Packet::Operation {
                    version: 1,
                    operator: Operator::Minimum,
                    operands: vec![Packet::Operation {
                        version: 5,
                        operator: Operator::Minimum,
                        operands: vec![Packet::Literal {
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
            Packet::Operation {
                version: 3,
                operator: Operator::Sum,
                operands: vec![
                    Packet::Operation {
                        version: 0,
                        operator: Operator::Sum,
                        operands: vec![
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
                    Packet::Operation {
                        version: 1,
                        operator: Operator::Sum,
                        operands: vec![
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
            Packet::Operation {
                version: 6,
                operator: Operator::Sum,
                operands: vec![
                    Packet::Operation {
                        version: 0,
                        operator: Operator::Sum,
                        operands: vec![
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
                    Packet::Operation {
                        version: 4,
                        operator: Operator::Sum,
                        operands: vec![
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
            Packet::Operation {
                version: 5,
                operator: Operator::Sum,
                operands: vec![Packet::Operation {
                    version: 1,
                    operator: Operator::Sum,
                    operands: vec![Packet::Operation {
                        version: 3,
                        operator: Operator::Sum,
                        operands: vec![
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

    #[test]
    fn test_packet_value_example1() {
        let mut stream = BitStream::from_str(EXAMPLE1);
        let packet = Packet::read(&mut stream);
        assert_eq!(packet.value(), 2021)
    }

    static PART2_EXAMPLES: [(&str, usize); 8] = [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    #[test]
    fn test_packet_value_part2_examples() {
        for (input, expected_result) in PART2_EXAMPLES {
            let mut stream = BitStream::from_str(input);
            let packet = Packet::read(&mut stream);
            assert_eq!(packet.value(), expected_result)
        }
    }
}
