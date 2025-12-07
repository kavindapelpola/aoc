use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::{map_res, opt, recognize},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn parse_int(input: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(one_of("+-")), digit1))), str::parse)(input)
}

fn parse_valve_name(input: &str) -> IResult<&str, &str> {
    recognize(alpha1)(input)
}

fn parse_valve_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(", "), parse_valve_name)(input)
}

pub fn parse_valve_details(input: &str) -> IResult<&str, (&str, i32)> {
    tuple((
        preceded(tag("Valve "), parse_valve_name),
        preceded(tag(" has flow rate="), parse_int),
    ))(input)
}

pub fn parse_valve_tunnels(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tuple((
            opt(tag("tunnels lead to valves ")),
            opt(tag("tunnel leads to valve ")),
        )),
        parse_valve_list,
    )(input)
}

pub fn parse_valve(input: &str) -> IResult<&str, ((&str, i32), Vec<&str>)> {
    separated_pair(parse_valve_details, tag("; "), parse_valve_tunnels)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_int() {
        let result = parse_int("2662540");
        assert_eq!(result.unwrap().1, 2662540);
    }

    #[test]
    fn test_parse_positive_int_with_plus() {
        let result = parse_int("+2662540");
        assert_eq!(result.unwrap().1, 2662540);
    }

    #[test]
    fn test_parse_negative_int() {
        let result = parse_int("-2662540");
        assert_eq!(result.unwrap().1, -2662540);
    }

    #[test]
    fn test_parse_valve_name() {
        let result = parse_valve_name("AV").unwrap();
        assert_eq!(result.1, "AV");
    }

    #[test]
    fn test_parse_valve_details() {
        let result = parse_valve_details("Valve AV has flow rate=0").unwrap();
        assert_eq!(result.1, ("AV", 0));
    }

    #[test]
    fn test_parse_valve_list_single_value() {
        let result = parse_valve_list("KV").unwrap();
        assert_eq!(result.1, vec!["KV"]);
    }

    #[test]
    fn test_parse_valve_list_multi_value() {
        let result = parse_valve_list("AX, PI").unwrap();
        assert_eq!(result.1, vec!["AX", "PI"]);
    }

    #[test]
    fn test_parse_valve_tunnels_multi_value() {
        let result = parse_valve_tunnels("tunnels lead to valves AX, PI").unwrap();
        assert_eq!(result.1, vec!["AX", "PI"]);
    }

    #[test]
    fn test_parse_valve_tunnels_single_value() {
        let result = parse_valve_tunnels("tunnel leads to valve KV").unwrap();
        assert_eq!(result.1, vec!["KV"]);
    }

    #[test]
    fn test_parse_valve_single_value() {
        let result = parse_valve("Valve OF has flow rate=19; tunnel leads to valve KV").unwrap();
        assert_eq!(result.1, (("OF", 19), vec!["KV"]));
    }

    #[test]
    fn test_parse_valve_multi_value() {
        let result =
            parse_valve("Valve AV has flow rate=0; tunnels lead to valves AX, PI").unwrap();
        assert_eq!(result.1, (("AV", 0), vec!["AX", "PI"]));
    }
}
