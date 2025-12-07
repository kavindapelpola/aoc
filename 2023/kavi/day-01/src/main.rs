mod load;
mod part_a;
mod part_b;

struct App {
    input: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            input: load::load(),
        }
    }

    fn word_digit(s: String) -> Option<char> {
        if s.contains("one") {
            return Some('1');
        }
        if s.contains("two") {
            return Some('2');
        }
        if s.contains("three") {
            return Some('3');
        }
        if s.contains("four") {
            return Some('4');
        }
        if s.contains("five") {
            return Some('5');
        }
        if s.contains("six") {
            return Some('6');
        }
        if s.contains("seven") {
            return Some('7');
        }
        if s.contains("eight") {
            return Some('8');
        }
        if s.contains("nine") {
            return Some('9');
        }
        None
    }

    fn first_word_digit(line: &String) -> Option<char> {
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                return line.chars().nth(i);
            }
            match App::word_digit(line[..=i].to_string()) {
                Some(c) => return Some(c),
                None => (),
            }
        }
        None
    }

    fn last_word_digit(line: &String) -> Option<char> {
        for i in 0..line.len() {
            let pos = line.len() - i - 1;
            if line.chars().nth(pos).unwrap().is_digit(10) {
                return line.chars().nth(pos);
            }
            match App::word_digit(line[pos..].to_string()) {
                Some(c) => return Some(c),
                None => (),
            }
        }
        None
    }

    pub fn digits(line: &String) -> u32 {
        let digits = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap()
    }

    pub fn word_digits(line: &String) -> u32 {
        format!(
            "{}{}",
            App::first_word_digit(&line).unwrap(),
            App::last_word_digit(&line).unwrap(),
        )
        .parse::<u32>()
        .unwrap()
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
