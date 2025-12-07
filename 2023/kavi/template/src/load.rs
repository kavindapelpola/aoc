use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

fn parser(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag("|"), tag("abc"))(s)
}

pub fn load() -> Vec<String> {
    let input = include_str!("../input.txt");
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
}
