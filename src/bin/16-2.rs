// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::collections::BinaryHeap;
// use std::io::Read;

struct BitStream {
    hex: Vec<u8>,
    start: usize,
}

impl BitStream {
    fn take_bit(&mut self) -> u64 {
        let hex_index = self.start / 4;
        let bit_index = 3 - self.start % 4;
        self.start += 1;
        let nibble = self.hex[hex_index] as u64;
        (nibble & 1 << bit_index) >> bit_index
    }

    fn take_bits(&mut self, n: usize) -> u64 {
        let mut bits = 0;
        for _ in 0..n {
            bits <<= 1;
            bits |= self.take_bit();
        }
        bits
    }

    fn take_int(&mut self) -> u64 {
        let mut n = 0;
        loop {
            let cont = self.take_bit();
            let nib = self.take_bits(4);
            n = (n << 4) + nib;
            if cont == 0 {
                break;
            }
        }
        n
    }
}

fn eval(bs: &mut BitStream) -> u64 {
    let _version = bs.take_bits(3);
    let type_id = bs.take_bits(3);
    if type_id == 4 {
        return bs.take_int();
    }
    let mut sub_values = Vec::new();
    let length_type = bs.take_bit();
    if length_type == 0 {
        let sub_bits = bs.take_bits(15);
        let sub_end = bs.start + sub_bits as usize;
        while bs.start < sub_end {
            sub_values.push(eval(bs));
        }
    } else {
        let sub_count = bs.take_bits(11);
        for _ in 0..sub_count {
            sub_values.push(eval(bs));
        }
    }
    match type_id {
        // Sum
        0 => sub_values.iter().sum(),
        // Product
        1 => sub_values.iter().product(),
        // Min
        2 => *sub_values.iter().min().unwrap(),
        // Max
        3 => *sub_values.iter().max().unwrap(),
        // greater than
        5 => {
            assert_eq!(2, sub_values.len());
            (sub_values[0] > sub_values[1]).into()
        }
        // less than
        6 => {
            assert_eq!(2, sub_values.len());
            (sub_values[0] < sub_values[1]).into()
        }
        // equal
        7 => {
            assert_eq!(2, sub_values.len());
            (sub_values[0] == sub_values[1]).into()
        }
        _ => panic!(),
    }
}

fn solve(hex_str: &str) -> u64 {
    let mut hex = Vec::with_capacity(hex_str.len());
    for b in hex_str.as_bytes() {
        hex.push(match *b {
            b'0'..=b'9' => (b - b'0') as u8,
            b'A'..=b'F' => 10 + (b - b'A') as u8,
            _ => panic!("{}: {}", hex_str, *b as char),
        });
    }
    eval(&mut BitStream { hex, start: 0 })
}

fn main() {
    dbg!(solve(include_str!("../../inputs/16.txt").trim()));
}

#[test]
fn test_examples() {
    assert_eq!(3, solve("C200B40A82"));
    assert_eq!(54, solve("04005AC33890"));
    assert_eq!(7, solve("880086C3E88112"));
    assert_eq!(9, solve("CE00C43D881120"));
    assert_eq!(1, solve("D8005AC2A8F0"));
    assert_eq!(0, solve("F600BC2D8F"));
    assert_eq!(0, solve("9C005AC2F8F0"));
    assert_eq!(1, solve("9C0141080250320F1802104A08"));
}
