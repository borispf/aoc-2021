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

    fn take_int(&mut self) {
        loop {
            let cont = self.take_bit();
            let _nib = self.take_bits(4);
            if cont == 0 {
                break;
            }
        }
    }
}

fn version_sum(bs: &mut BitStream) -> u64 {
    let version = bs.take_bits(3);
    let type_id = bs.take_bits(3);
    let mut vsum = version;
    if type_id == 4 {
        bs.take_int();
    } else {
        let length_type = bs.take_bit();
        if length_type == 0 {
            let sub_bits = bs.take_bits(15);
            let sub_end = bs.start + sub_bits as usize;
            while bs.start < sub_end {
                vsum += version_sum(bs);
            }
        } else {
            let sub_count = bs.take_bits(11);
            for _ in 0..sub_count {
                vsum += version_sum(bs);
            }
        }
    }
    vsum
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
    version_sum(&mut BitStream { hex, start: 0 })
}

fn main() {
    dbg!(solve(include_str!("../../inputs/16.txt").trim()));
}

#[test]
fn test_examples() {
    assert_eq!(6, solve("D2FE28"));
    assert_eq!(9, solve("38006F45291200"));
    assert_eq!(16, solve("8A004A801A8002F478"));
    assert_eq!(12, solve("620080001611562C8802118E34"));
    assert_eq!(23, solve("C0015000016115A2E0802F182340"));
    assert_eq!(31, solve("A0016C880162017C3686B18A3D4780"));
}
