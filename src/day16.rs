use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::From;

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> String {
    input
        .chars()
        .map(|x| {
            let raw = u64::from_str_radix(&x.to_string(), 16).unwrap(); // parse hex digit
            let bits = format!("{:04b}", raw); // convert it to a binary string
            bits
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PacketType {
    Literal,
    Operator(u8),
}

impl From<u8> for PacketType {
    fn from(x: u8) -> Self {
        match x {
            4 => Self::Literal,
            _ => Self::Operator(x),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Length {
    Bits(usize),  // how many bits subpackets take
    Count(usize), // how many subpackets
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: u8,
    type_id: PacketType,
    subpackets: Vec<Packet>,
    value: Option<u128>, // bold assumption, but…
    length_id: Option<Length>,
}

impl From<&str> for Packet {
    fn from(raw: &str) -> Self {
        let version = u8::from_str_radix(&raw[0..3], 2).unwrap();
        let type_id = PacketType::from(u8::from_str_radix(&raw[3..6], 2).unwrap());
        let length_id = match type_id {
            PacketType::Literal => None,
            PacketType::Operator(_) => Some(match raw.chars().nth(6) {
                Some('0') => Length::Bits(usize::from_str_radix(&raw[7..7 + 15], 2).unwrap()),
                Some('1') => Length::Count(usize::from_str_radix(&raw[7..7 + 11], 2).unwrap()),
                _ => panic!("Unrecognized length id"),
            }),
        };

        Packet {
            version,
            type_id,
            subpackets: vec![],
            value: None,
            length_id,
        }
    }
}

fn parse_message(message: &str) -> Vec<Packet> {
    const MIN_LENGTH: usize = 6 + 1 + 4; // minimum length of packet is a literal with 4 bits
    let mut packets = Vec::<Packet>::new();
    let mut offset = 0;

    while offset < message.len() - MIN_LENGTH - 1 {
        let packet_type =
            PacketType::from(u8::from_str_radix(&message[offset + 3..offset + 6], 2).unwrap());
        let packet_end = match packet_type {
            PacketType::Literal => {
                let mut i = offset + 6;
                while message.chars().nth(i) == Some('1') {
                    i += 5;
                }
                i += 5;
                i
            }
            PacketType::Operator(_) => match message.chars().nth(offset + 6) {
                Some('0') => offset + 7 + 15,
                Some('1') => offset + 7 + 11,
                _ => panic!("Unrecognized length type"),
            },
        };

        let packet = Packet::from(&message[offset..packet_end]);
        packets.push(packet);
        offset = packet_end as usize
    }

    packets
}

#[aoc(day16, part1)]
pub fn solve_part1(message: &str) -> u64 {
    parse_message(message)
        .iter()
        .map(|packet| packet.version as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            Packet::from("110100101111111000101000"),
            Packet {
                version: 6,
                type_id: PacketType::Literal,
                subpackets: vec![],
                value: None,
                length_id: None,
            }
        );
    }

    #[test]
    fn test_parse_operator_length_bits() {
        assert_eq!(
            Packet::from("00111000000000000110111101000101001010010001001000000000"),
            Packet {
                version: 1,
                type_id: PacketType::Operator(6),
                length_id: Some(Length::Bits(27)),
                subpackets: vec![
                    // Packet::from("11010001010"),
                    // Packet::from("0101001000100100"),
                ],
                value: None,
            }
        );
    }

    #[test]
    fn test_parse_operator_length_count() {
        assert_eq!(
            Packet::from("11101110000000001101010000001100100000100011000001100000"),
            Packet {
                version: 7,
                type_id: PacketType::Operator(3),
                length_id: Some(Length::Count(3)),
                subpackets: vec![
                    // Packet::from("01010000001"),
                    // Packet::from("10010000010"),
                    // Packet::from("00110000011"),
                ],
                value: None,
            }
        )
    }

    #[test]
    fn test_day16_parse_input() {
        assert_eq!(parse_input("F"), "1111");
        assert_eq!(parse_input("2"), "0010");
        assert_eq!(parse_input("A8"), "10101000");
    }

    #[test]
    fn test_day16_solve_part1() {
        assert_eq!(solve_part1(&parse_input("8A004A801A8002F478")), 16);
        assert_eq!(solve_part1(&parse_input("620080001611562C8802118E34")), 12);
        assert_eq!(
            solve_part1(&parse_input("C0015000016115A2E0802F182340")),
            23
        );
        assert_eq!(
            solve_part1(&parse_input("A0016C880162017C3686B18A3D4780")),
            31
        );
    }
}
