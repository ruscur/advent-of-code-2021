use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let digits = lines
        .get(0)
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(16))
        .map(|n| format!("{:04b}", n))
        .map(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(2))
                .map(|d| d as usize)
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect::<Vec<usize>>();

    Ok(digits)
}

fn bits_to_val(bits: &Vec<usize>) -> usize {
    usize::from_str_radix(
        &bits
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(""),
        2,
    )
    .unwrap()
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum PacketType {
    Literal(Vec<usize>),
    Operator((OperatorType, Vec<Packet>)),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    ptype: PacketType,
    size: usize,
}

impl Packet {
    fn new(bits: Vec<usize>) -> Packet {
        let version = bits[0] << 2 | bits[1] << 1 | bits[2];
        let type_id = bits[3] << 2 | bits[4] << 1 | bits[5];

        let (ptype, size) = if type_id == 4 {
            let mut offset = 6;
            let mut value: Vec<usize> = vec![];

            loop {
                value.extend(bits[offset + 1..=offset + 4].to_vec());
                offset += 5;

                // Was this our last packet?
                if bits[offset - 5] == 0 {
                    break;
                }
            }

            (PacketType::Literal(value), offset)
        } else {
            let operator_type = match type_id {
                0 => OperatorType::Sum,
                1 => OperatorType::Product,
                2 => OperatorType::Minimum,
                3 => OperatorType::Maximum,
                5 => OperatorType::GreaterThan,
                6 => OperatorType::LessThan,
                7 => OperatorType::EqualTo,
                _ => {
                    println!("Got type id {}!", type_id);
                    panic!()
                }
            };

            let mut subpackets = vec![];
            let mut offset = 6;
            match bits[offset] {
                1 => {
                    // The next 11 bits tell us how many subpackets there are
                    let subpacket_count = bits_to_val(&bits[7..18].to_vec());
                    // The first subpacket begins 1 bit after the count
                    offset += 12;
                    while subpackets.len() < subpacket_count {
                        let new_packet = Packet::new(bits[offset..].to_vec());
                        offset += new_packet.size;
                        subpackets.push(new_packet);
                    }
                }
                _ => {
                    // The next 15 bits are a value for how large the remaining packets are
                    let subpacket_size = bits_to_val(&bits[7..22].to_vec());
                    // The first subpacket begins 1 bit after the size
                    offset += 16;

                    let end = offset + subpacket_size;
                    while offset < end {
                        let new_packet = Packet::new(bits[offset..end].to_vec());
                        offset += new_packet.size;
                        subpackets.push(new_packet);
                    }
                }
            }
            (PacketType::Operator((operator_type, subpackets)), offset)
        };

        Packet {
            version,
            ptype,
            size,
        }
    }

    fn version_sum(&self) -> usize {
        match &self.ptype {
            PacketType::Literal(_) => self.version,
            PacketType::Operator(sub) => {
                self.version + sub.1.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }

    fn value(&self) -> usize {
        match &self.ptype {
            PacketType::Literal(val) => bits_to_val(&val),
            PacketType::Operator((op, packets)) => match op {
                OperatorType::Sum => packets.iter().map(|p| p.value()).sum(),
                OperatorType::Product => packets.iter().fold(1, |acc, p| acc * p.value()),
                OperatorType::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
                OperatorType::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
                OperatorType::GreaterThan => (packets[0].value() > packets[1].value()) as usize,
                OperatorType::LessThan => (packets[0].value() < packets[1].value()) as usize,
                OperatorType::EqualTo => (packets[0].value() == packets[1].value()) as usize,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let bits = parse(lines?)?;

    let packet = Packet::new(bits);

    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.value());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let bits = parse(vec!["D2FE28".to_string()]).unwrap();

        let packet = Packet::new(bits);

        dbg!(&packet);
        match &packet.ptype {
            PacketType::Literal(val) => assert!(bits_to_val(&val) == 2021),
            PacketType::Operator(_) => panic!(),
        }
    }

    #[test]
    fn example_2() {
        let bits = parse(vec!["38006F45291200".to_string()]).unwrap();

        let packet = Packet::new(bits);

        dbg!(&packet);
    }

    #[test]
    fn example_3() {
        let bits = parse(vec!["8A004A801A8002F478".to_string()]).unwrap();

        let packet = Packet::new(bits);

        if packet.version != 4 {
            panic!();
        }
        match &packet.ptype {
            PacketType::Literal(_) => panic!(),
            PacketType::Operator(subpackets) => {
                let sub = subpackets.1.get(0).unwrap();
                if sub.version != 1 {
                    panic!();
                }
                match &sub.ptype {
                    PacketType::Literal(_) => panic!(),
                    PacketType::Operator(subpackets) => {
                        let sub = subpackets.1.get(0).unwrap();
                        if sub.version != 5 {
                            panic!();
                        }
                        match &sub.ptype {
                            PacketType::Operator(subpackets) => {
                                let sub = subpackets.1.get(0).unwrap();
                                if sub.version != 6 {
                                    panic!();
                                }
                                match &sub.ptype {
                                    PacketType::Literal(val) => {
                                        println!("{}", bits_to_val(&val));
                                    }
                                    PacketType::Operator(_) => panic!(),
                                }
                            }
                            PacketType::Literal(_) => panic!(),
                        }
                    }
                }
            }
        }

        dbg!(&packet);
        assert!(packet.version_sum() == 16);
    }

    #[test]
    fn example_4() {
        let bits = parse(vec!["620080001611562C8802118E34".to_string()]).unwrap();

        let packet = Packet::new(bits);

        println!("{:?}", packet);

        assert!(packet.version_sum() == 12);
    }

    #[test]
    fn example_5() {
        let bits = parse(vec!["C0015000016115A2E0802F182340".to_string()]).unwrap();

        let packet = Packet::new(bits);

        println!("{:?}", packet);

        assert!(packet.version_sum() == 23);
    }

    #[test]
    fn example_6() {
        let bits = parse(vec!["A0016C880162017C3686B18A3D4780".to_string()]).unwrap();

        let packet = Packet::new(bits);

        println!("{:?}", packet);

        assert!(packet.version_sum() == 31);
    }

    #[test]
    fn example_7() {
        let bits = parse(vec!["C200B40A82".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 3);
    }

    #[test]
    fn example_8() {
        let bits = parse(vec!["04005AC33890".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 54);
    }

    #[test]
    fn example_9() {
        let bits = parse(vec!["880086C3E88112".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 7);
    }

    #[test]
    fn example_10() {
        let bits = parse(vec!["CE00C43D881120".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 9);
    }

    #[test]
    fn example_11() {
        let bits = parse(vec!["D8005AC2A8F0".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 1);
    }

    #[test]
    fn example_12() {
        let bits = parse(vec!["F600BC2D8F".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 0);
    }

    #[test]
    fn example_13() {
        let bits = parse(vec!["9C005AC2F8F0".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 0);
    }

    #[test]
    fn example_14() {
        let bits = parse(vec!["9C0141080250320F1802104A08".to_string()]).unwrap();

        let packet = Packet::new(bits);

        assert!(packet.value() == 1);
    }
}
