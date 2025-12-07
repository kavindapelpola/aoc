#[derive(Debug, PartialEq)]
pub enum Data {
    Number(u8),
    Packet(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Data::Number(l), Data::Number(r)) => l.partial_cmp(r),
            (Data::Packet(l), Data::Packet(r)) => {
                if l.len() == 0 && r.len() == 0 {
                    return Some(std::cmp::Ordering::Equal);
                }
                if l.len() == 0 && r.len() > 0 {
                    return Some(std::cmp::Ordering::Less);
                }
                if l.len() > 0 && r.len() == 0 {
                    return Some(std::cmp::Ordering::Greater);
                }
                // else l.len() > 0 && r.len() > 0
                if l[0] < r[0] {
                    return Some(std::cmp::Ordering::Less);
                } else if l[0] > r[0] {
                    return Some(std::cmp::Ordering::Greater);
                }
                // else l[0] == r[0]
                return l.partial_cmp(r);
            }
            (Data::Packet(l), Data::Number(r)) => {
                if l.len() == 0 {
                    return Some(std::cmp::Ordering::Less);
                }
                return l[0].partial_cmp(&Data::Number(*r));
            }
            (Data::Number(l), Data::Packet(r)) => {
                if r.len() == 0 {
                    return Some(std::cmp::Ordering::Greater);
                }
                return Data::Packet(vec![Data::Number(*l)]).partial_cmp(&r[0]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_numbers() {
        assert_eq!(Data::Number(1) < Data::Number(2), true);
        assert_eq!(Data::Number(1) > Data::Number(2), false);
        assert_eq!(Data::Number(2) < Data::Number(1), false);
        assert_eq!(Data::Number(2) > Data::Number(1), true);
        assert_eq!(Data::Number(1) < Data::Number(1), false);
    }

    #[test]
    fn equality_numbers() {
        assert_eq!(Data::Number(1) == Data::Number(1), true);
        assert_eq!(Data::Number(1) == Data::Number(2), false);
    }

    #[test]
    fn compare_packets_same_length() {
        assert_eq!(
            Data::Packet(vec![Data::Number(1)]) < Data::Packet(vec![Data::Number(2)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(2)]) < Data::Packet(vec![Data::Number(1)]),
            false
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1)]) < Data::Packet(vec![Data::Number(1)]),
            false
        );
    }

    #[test]
    fn lhs_packet_longer_but_lower_number_is_less() {
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(1)])
                < Data::Packet(vec![Data::Number(2)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(1), Data::Number(1)])
                < Data::Packet(vec![Data::Number(1), Data::Number(2)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(2), Data::Number(1)])
                < Data::Packet(vec![Data::Number(1), Data::Number(2)]),
            false
        );
    }

    #[test]
    fn lhs_packet_longer_but_higher_number_is_greater() {
        assert_eq!(
            Data::Packet(vec![Data::Number(3), Data::Number(1)])
                < Data::Packet(vec![Data::Number(2)]),
            false
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(3), Data::Number(1)])
                > Data::Packet(vec![Data::Number(2)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(3), Data::Number(1)])
                > Data::Packet(vec![Data::Number(3)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(3)])
                > Data::Packet(vec![Data::Number(3), Data::Number(1)]),
            false
        );
    }

    #[test]
    fn equality_packets_same_length() {
        assert_eq!(
            Data::Packet(vec![Data::Number(1)]) == Data::Packet(vec![Data::Number(1)]),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1)]) == Data::Packet(vec![Data::Number(2)]),
            false
        );
    }

    #[test]
    fn compare_packet_with_number() {
        assert_eq!(Data::Packet(vec![Data::Number(1)]) < Data::Number(2), true);
        assert_eq!(Data::Packet(vec![Data::Number(2)]) < Data::Number(1), false);
        assert_eq!(Data::Packet(vec![Data::Number(1)]) < Data::Number(1), false);
    }

    #[test]
    fn compare_packet_long_with_number_matches_first() {
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(2)]) > Data::Number(1),
            false
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(2)]) < Data::Number(1),
            false
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(2), Data::Number(2)]) > Data::Number(1),
            true
        );
        assert_eq!(
            Data::Packet(vec![Data::Number(1), Data::Number(2)]) < Data::Number(2),
            true
        );
    }
}
