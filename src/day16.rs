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
    Bits(usize), // how many bits subpackets take
                 // Count(usize), // how many subpackets
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: u8,
    type_id: PacketType,
    subpackets: Vec<Packet>,
    content: u128, // bold assumption, but…
}

impl From<&str> for Packet {
    fn from(raw: &str) -> Self {
        println!("Creating {:?}", raw);
        let version = u8::from_str_radix(&raw[0..3], 2).unwrap();
        let type_id = PacketType::from(u8::from_str_radix(&raw[3..6], 2).unwrap());

        Packet {
            version,
            type_id,
            subpackets: vec![],
            content: 0,
        }
    }
}

fn parse_message(message: &str) -> Vec<Packet> {
    let mut packets = Vec::<Packet>::new();
    let mut offset = 0;
    while offset < message.len() - 1 {
        println!(
            "header: {} offset={}",
            message[offset..offset + 6].to_string(),
            offset
        );
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
            PacketType::Operator(_) => {
                let _length = match message.chars().nth(offset + 6) {
                    Some('0') => Length::Bits(
                        usize::from_str_radix(&message[offset + 7..offset + 7 + 15], 2).unwrap(),
                    ),
                    Some('1') => Length::Bits(
                        usize::from_str_radix(&message[offset + 7..offset + 7 + 11], 2).unwrap(),
                    ),
                    _ => panic!("Unrecognized length type"),
                };

                0
            }
        };

        println!("offset={} packet_end={}", offset, packet_end);
        let packet = Packet::from(&message[offset..packet_end]);
        packets.push(packet);
        offset = 4 * (packet_end as f64 / 4f64).ceil() as usize;
    }

    packets
}

#[aoc(day16, part1)]
pub fn solve_part1(_message: &str) -> u64 {
    // tokenize(message).into_iter().map(|raw| )

    0
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
                content: 0
            }
        )
    }

    #[test]
    fn test_parse_message() {
        let input = "1101001011111110001010000011001001001010"; // lit: 2021 lit: 42
        assert_eq!(
            parse_message(input),
            vec![
                Packet {
                    version: 6,
                    type_id: PacketType::Literal,
                    subpackets: vec![],
                    content: 0
                },
                Packet {
                    version: 1,
                    type_id: PacketType::Literal,
                    subpackets: vec![],
                    content: 0
                }
            ]
        );
    }

    #[test]
    fn test_day16_parse_input() {
        assert_eq!(parse_input("F"), "1111");
        assert_eq!(parse_input("2"), "0010");
        assert_eq!(parse_input("A8"), "10101000");
    }

    #[test]
    fn test_day16_solve_part1() {
        assert_eq!(solve_part1("8A004A801A8002F478"), 16);
        assert_eq!(solve_part1("620080001611562C8802118E34"), 12);
        assert_eq!(solve_part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(solve_part1("A0016C880162017C3686B18A3D4780"), 31);
    }
}
