use aoc_runner_derive::aoc;
use nom::bits::bits;
use nom::bits::complete::{tag, take};
use nom::branch::alt;
use nom::combinator::{flat_map, map, value};
use nom::error::{context, Error};
use nom::multi::{fold_many0, length_count, many1};
use nom::sequence::preceded;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum PacketTy {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    ty: PacketTy,
    value: usize,
    children: Vec<Packet>,
}

impl Packet {
    fn evaluate(&self) -> usize {
        let v = match self.ty {
            PacketTy::Literal => self.value,
            PacketTy::Sum => self.children.iter().map(Packet::evaluate).sum(),
            PacketTy::Product => self.children.iter().map(Packet::evaluate).product(),
            PacketTy::Minimum => self.children.iter().map(Packet::evaluate).min().unwrap(),
            PacketTy::Maximum => self.children.iter().map(Packet::evaluate).max().unwrap(),
            PacketTy::GreaterThan => {
                assert!(self.children.len() == 2);
                if self.children[0].evaluate() > self.children[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketTy::LessThan => {
                assert!(self.children.len() == 2);
                if self.children[0].evaluate() < self.children[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketTy::EqualTo => {
                assert!(self.children.len() == 2);
                if self.children[0].evaluate() == self.children[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        };
        v
    }
}

type Input<'a> = (&'a [u8], usize);

fn packet_ty<'a>(input: Input<'a>) -> IResult<Input<'a>, PacketTy> {
    alt((
        value(PacketTy::Sum, tag(0, 3usize)),
        value(PacketTy::Product, tag(1, 3usize)),
        value(PacketTy::Minimum, tag(2, 3usize)),
        value(PacketTy::Maximum, tag(3, 3usize)),
        value(PacketTy::Literal, tag(4, 3usize)),
        value(PacketTy::GreaterThan, tag(5, 3usize)),
        value(PacketTy::LessThan, tag(6, 3usize)),
        value(PacketTy::EqualTo, tag(7, 3usize)),
    ))(input)
}

fn packet(input: &[u8]) -> IResult<&[u8], Packet> {
    bits::<_, _, Error<(&[u8], usize)>, _, _>(context("packet", packet_bits))(input)
}

fn packet_bits<'a>(input: Input<'a>) -> IResult<Input<'a>, Packet> {
    fn bitlen_children<'b>(input: Input<'b>) -> IResult<Input<'b>, Vec<Packet>> {
        let ((input, offset), len): (_, usize) = context("bitlen_children", take(15usize))(input)?;
        let byte_count = usize::min((len + offset + 7) / 8, input.len());
        let (_, children) = many1(packet_bits)((&input[0..byte_count], offset))?;
        Ok(((&input[(len + offset) / 8..], (len + offset) % 8), children))
    }

    let group = |val, item: u8| (val << 4) | (item as usize);

    let (input, version): (_, u8) = take(3usize)(input)?;
    let (input, ty) = packet_ty(input)?;
    match ty {
        PacketTy::Literal => {
            let (input, value) = flat_map(
                fold_many0(preceded(tag(1, 1usize), take(4usize)), || 0usize, group),
                |v| preceded(tag(0, 1usize), map(take(4usize), move |x| group(v, x))),
            )(input)?;
            Ok((
                input,
                Packet {
                    version,
                    ty,
                    value,
                    children: vec![],
                },
            ))
        }
        _ => {
            let (input, children) = alt((
                preceded(
                    tag(0, 1usize), // total len
                    bitlen_children,
                ),
                preceded(
                    tag(1, 1usize), // num children
                    length_count(take::<_, usize, _, _>(11usize), packet_bits),
                ),
            ))(input)?;
            Ok((
                input,
                Packet {
                    version,
                    ty,
                    value: 0,
                    children,
                },
            ))
        }
    }
}

fn version_sum(total: usize, packet: &Packet) -> usize {
    packet
        .children
        .iter()
        .fold(total + packet.version as usize, version_sum)
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let input: Vec<u8> = input
        .chars()
        .map(|x| x.to_digit(16).unwrap() as u8)
        .collect();
    let input: Vec<u8> = input.chunks(2).map(|win| (win[0] << 4) | win[1]).collect();
    let (_, packet) = packet(&input).unwrap();
    version_sum(0, &packet)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let input: Vec<u8> = input
        .chars()
        .map(|x| x.to_digit(16).unwrap() as u8)
        .collect();
    let input: Vec<u8> = input.chunks(2).map(|win| (win[0] << 4) | win[1]).collect();
    let (_, expression) = packet(&input).unwrap();
    expression.evaluate()
}

// #[test]
// fn test_part1() {
//     let input = include_str!("../input/2021/sample16.txt");
//     assert_eq!(part1(&input), 40);
// }

// #[test]
// fn test_part2() {
//     let input = include_str!("../input/2021/sample16.txt");
//     assert_eq!(part2(&input), 2188189693529);
// }
