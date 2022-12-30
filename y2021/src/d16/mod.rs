use bitreader::BitReader;
use hex;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: PacketTypeId,
    data: Data,
}

#[derive(FromPrimitive, Debug, Clone, Eq, PartialEq)]
enum PacketTypeId {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

#[derive(Debug)]
enum Data {
    Literal(u64), // Dunno how long this can be
    Operator(Length, Vec<Packet>),
}

#[derive(Debug)]
enum Length {
    // 0
    Bits(u64), // 15 bits to get total subsequent length
    // 1
    SubPackets(u16), // 11 bits to get num subsequent sub_packets
}

impl Length {
    fn has_more(&self, num_sub_packets_parsed: u16, num_bits_parsed: u64) -> bool {
        match self {
            Length::Bits(total_bits) => num_bits_parsed < *total_bits,
            Length::SubPackets(total_sub_packets) => num_sub_packets_parsed < *total_sub_packets,
        }
    }
}

fn parse_packet(bit_reader: &mut BitReader) -> Packet {
    let version = bit_reader.read_u8(3).unwrap();
    let type_id = bit_reader.read_u8(3).unwrap();
    let type_id = PacketTypeId::from_u8(type_id).unwrap();
    let sub_packets = match type_id {
        PacketTypeId::Literal => parse_literal(bit_reader),
        _ => parse_operator(bit_reader),
    };
    Packet {
        version,
        type_id,
        data: sub_packets,
    }
}

fn parse_literal(bit_reader: &mut BitReader) -> Data {
    let mut literal = 0u64;
    let mut has_more = true;
    while has_more {
        has_more = bit_reader.read_bool().unwrap();
        literal = (literal << 4) + bit_reader.read_u64(4).unwrap();
    }
    Data::Literal(literal)
}

fn parse_operator(bit_reader: &mut BitReader) -> Data {
    // Match is a bit funky, but easier to keep the "0 then 1" order
    let length = match bit_reader.read_bool().unwrap() {
        // Next 15 bytes indicate bit length
        false => Length::Bits(bit_reader.read_u64(15).unwrap()),
        true => Length::SubPackets(bit_reader.read_u16(11).unwrap()),
    };
    let sub_packet_start = bit_reader.position();
    let mut num_sub_packets = 0;
    let mut sub_packets = Vec::new();
    while length.has_more(num_sub_packets, bit_reader.position() - sub_packet_start) {
        sub_packets.push(parse_packet(bit_reader));
        num_sub_packets += 1;
    }
    Data::Operator(length, sub_packets)
}

fn sum_version_numbers(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.data {
            Data::Literal(_) => 0,
            Data::Operator(_, sub_packets) => sub_packets.iter().map(sum_version_numbers).sum(),
        }
}

fn bool_op(evaluated: impl Iterator<Item = u64>, operator: impl FnOnce(&u64, &u64) -> bool) -> u64 {
    let (first, second) = evaluated.collect_tuple::<(u64, u64)>().unwrap();
    if operator(&first, &second) {
        1
    } else {
        0
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.data {
        Data::Literal(v) => *v,
        Data::Operator(_, sub_packets) => {
            let evaluated = sub_packets.iter().map(evaluate);
            match packet.type_id {
                PacketTypeId::Sum => evaluated.sum(),
                PacketTypeId::Product => evaluated.product(),
                PacketTypeId::Minimum => evaluated.min().unwrap(),
                PacketTypeId::Maximum => evaluated.max().unwrap(),
                // TODO - Would be great to not even have this possibility in the type system.
                PacketTypeId::Literal => panic!("Should not be able to get here with an operator!"),
                PacketTypeId::GreaterThan => bool_op(evaluated, u64::gt),
                PacketTypeId::LessThan => bool_op(evaluated, u64::lt),
                PacketTypeId::EqualTo => bool_op(evaluated, u64::eq),
            }
        }
    }
}

// TODO - Use proper Results.
pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let bytes = hex::decode(input).unwrap();
    let packet = parse_packet(&mut BitReader::new(&bytes));

    println!("Part 1: {}", sum_version_numbers(&packet));
    println!("Part 2: {}", evaluate(&packet));
}
