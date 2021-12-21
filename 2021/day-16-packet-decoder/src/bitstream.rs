use std::str::Chars;

pub struct BitStream<'a> {
    input: Chars<'a>,
    have_bits: usize,
    value: usize,
    bits_read: usize,
}

impl BitStream<'_> {
    pub fn new(input: Chars) -> BitStream {
        BitStream {
            input: input,
            have_bits: 0,
            value: 0,
            bits_read: 0,
        }
    }

    pub fn read(&mut self, bits_to_read: usize) -> usize {
        println!("read: {}", bits_to_read);

        while self.have_bits < bits_to_read {
            let nibble = self.pop_char().unwrap();
            self.value = (self.value << 4) | nibble;
            self.have_bits += 4;
        }

        if self.have_bits == bits_to_read {
            // special "aligned" case, can just return value!
            let result = self.value;
            self.have_bits = 0;
            self.value = 0;
            self.bits_read += bits_to_read;
            println!("aligned read({}), result={:b}", bits_to_read, result);
            result
        } else if self.have_bits > bits_to_read {
            let extra_bits: usize = self.have_bits - bits_to_read;
            let result = self.value >> extra_bits;
            let rem_shift = 4 - extra_bits;
            let remainder = (self.value << rem_shift & 15) >> rem_shift;
            self.have_bits = extra_bits;
            self.value = remainder;
            self.bits_read += bits_to_read;
            println!(
                "normal read({}), result={:b}, extra_bits={}, remainder={:b}",
                bits_to_read, result, extra_bits, remainder
            );
            result
        } else {
            panic!("Unreachable");
        }
    }

    fn pop_char(&mut self) -> Option<usize> {
        match self.input.next() {
            Some(c) => c.to_digit(16).map(|v| v as usize),
            None => None,
        }
    }

    pub fn bits_read(&self) -> usize {
        self.bits_read
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "D2FE28";
    static EXAMPLE2: &str = "38006F45291200";
    static EXAMPLE3: &str = "EE00D40C823060";

    #[test]
    fn test_bitstream_example1() {
        let mut stream = BitStream::new(EXAMPLE1.chars());
        assert_eq!(stream.read(3), 6);
        assert_eq!(stream.read(3), 4);
        assert_eq!(stream.read(1), 1);
        assert_eq!(stream.read(4), 7);
        assert_eq!(stream.read(1), 1);
        assert_eq!(stream.read(4), 14);
        assert_eq!(stream.read(1), 0);
        assert_eq!(stream.read(4), 5);
        assert_eq!(stream.bits_read(), 21);
    }

    #[test]
    fn test_bitstream_example2() {
        let mut stream = BitStream::new(EXAMPLE2.chars());
        assert_eq!(stream.read(3), 1);
        assert_eq!(stream.read(3), 6);
        assert_eq!(stream.read(1), 0);
        assert_eq!(stream.read(15), 27);
        assert_eq!(stream.read(11), 0b11010001010);
        assert_eq!(stream.read(16), 0b0101001000100100);
        assert_eq!(stream.bits_read(), 49);
    }

    #[test]
    fn test_bitstream_example3() {
        let mut stream = BitStream::new(EXAMPLE3.chars());
        assert_eq!(stream.read(3), 7);
        assert_eq!(stream.read(3), 3);
        assert_eq!(stream.read(1), 1);
        assert_eq!(stream.read(11), 3);
        assert_eq!(stream.read(11), 0b01010000001);
        assert_eq!(stream.read(11), 0b10010000010);
        assert_eq!(stream.read(11), 0b00110000011);
        assert_eq!(stream.bits_read(), 51);
    }
}
