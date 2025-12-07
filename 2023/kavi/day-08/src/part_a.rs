use crate::App;

pub fn run() {
    let inst = "LRLRRRLRLLRRLRLRRRLRLRRLRRLLRLRRLRRLRRRLRRRLRLRRRLRLRRLRRLLRLRLLLLLRLRLRRLLRRRLLLRLLLRRLLLLLRLLLRLRRLRRLRRRLRRRLRRLRRLRRRLRLRLRRLRLRLRLRRLRRRLLRLLRRLRLRRRLRLRRRLRLRRRLRRRLRRLRLLLLRLRRRLRLRRLRLRRLRRLRRLLRRRLLLLLLRLRRRLRRLLRRRLRRLLLRLRLRLRRRLRRLRLRRRLRRLRRRLLRRLRRLLLRRRR".to_string();
    let mut pos = "AAA".to_string();
    let app = App::new();
    let mut steps = 0;
    loop {
        for c in inst.chars() {
            let (left, right) = app.input.get(&pos).unwrap();
            pos = if c == 'L' { left } else { right }.to_string();
            steps += 1;
            if pos == "ZZZ" {
                println!("Part A: {:?}", steps);
                return;
            }
        }
    }
}
