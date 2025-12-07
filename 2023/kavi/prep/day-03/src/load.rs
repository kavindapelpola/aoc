use crate::Data;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

fn packet_data(s: &str) -> IResult<&str, Vec<Data>> {
    separated_list0(
        tag(","),
        alt((
            map(digit1, |d: &str| Data::Number(d.parse().unwrap())),
            map(packet, |p| Data::Packet(p)),
        )),
    )(s)
}

fn packet(s: &str) -> IResult<&str, Vec<Data>> {
    delimited(tag("["), packet_data, tag("]"))(s)
}

fn packets(s: &str) -> IResult<&str, (Vec<Data>, Vec<Data>)> {
    separated_pair(packet, tag("\n"), packet)(s)
}

fn parser(s: &str) -> IResult<&str, Vec<(Vec<Data>, Vec<Data>)>> {
    separated_list0(tag("\n\n"), packets)(s)
}

pub fn load() -> Vec<(Vec<Data>, Vec<Data>)> {
    let input = include_str!("../input.txt");

    parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_returns_packet_data() {
        assert_eq!(
            parser(
                r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]"#
            ),
            Ok((
                "",
                vec![
                    (
                        vec![
                            Data::Number(1),
                            Data::Number(1),
                            Data::Number(3),
                            Data::Number(1),
                            Data::Number(1)
                        ],
                        vec![
                            Data::Number(1),
                            Data::Number(1),
                            Data::Number(5),
                            Data::Number(1),
                            Data::Number(1)
                        ]
                    ),
                    (
                        vec![
                            Data::Packet(vec![Data::Number(1)]),
                            Data::Packet(vec![Data::Number(2), Data::Number(3), Data::Number(4),])
                        ],
                        vec![Data::Packet(vec![Data::Number(1)]), Data::Number(4)]
                    )
                ]
            ))
        );
    }

    #[test]
    fn example1() {
        let (_, lhs) = packet("[1,1,3,1,1]").unwrap();
        let (_, rhs) = packet("[1,1,5,1,1]").unwrap();
        assert!(lhs < rhs);
    }

    #[test]
    fn example2() {
        let (_, lhs) = packet("[[1],[2,3,4]]").unwrap();
        let (_, rhs) = packet("[[1],4]").unwrap();
        assert!(lhs < rhs);
    }

    #[test]
    fn example3a() {
        let (_, lhs) = packet("[[8,7,6]]").unwrap();
        let (_, rhs) = packet("[9]").unwrap();
        assert!(lhs < rhs);
    }

    #[test]
    fn example3b() {
        let (_, lhs) = packet("[9]").unwrap();
        let (_, rhs) = packet("[[8,7,6]]").unwrap();
        assert!(lhs > rhs);
    }

    #[test]
    fn example4() {
        let (_, lhs) = packet("[[4,4],4,4]").unwrap();
        let (_, rhs) = packet("[[4,4],4,4,4]").unwrap();
        assert!(lhs < rhs);
    }

    #[test]
    fn example5() {
        let (_, lhs) = packet("[7,7,7,7]").unwrap();
        let (_, rhs) = packet("[7,7,7]").unwrap();
        assert!(lhs > rhs);
    }

    #[test]
    fn example6() {
        let (_, lhs) = packet("[]").unwrap();
        let (_, rhs) = packet("[3]").unwrap();
        assert!(lhs < rhs);
    }

    #[test]
    fn example7() {
        let (_, lhs) = packet("[[[]]]").unwrap();
        let (_, rhs) = packet("[[]]").unwrap();
        assert!(lhs > rhs);
    }

    #[test]
    fn example8() {
        let (_, lhs) = packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let (_, rhs) = packet("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(lhs > rhs);
    }
}
