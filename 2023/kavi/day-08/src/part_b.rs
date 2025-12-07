use std::collections::HashMap;

use crate::App;

pub fn run() {
    let inst = "LRLRRRLRLLRRLRLRRRLRLRRLRRLLRLRRLRRLRRRLRRRLRLRRRLRLRRLRRLLRLRLLLLLRLRLRRLLRRRLLLRLLLRRLLLLLRLLLRLRRLRRLRRRLRRRLRRLRRLRRRLRLRLRRLRLRLRLRRLRRRLLRLLRRLRLRRRLRLRRRLRLRRRLRRRLRRLRLLLLRLRRRLRLRRLRLRRLRRLRRLLRRRLLLLLLRLRRRLRRLLRRRLRRLLLRLRLRLRRRLRRLRLRRRLRRLRRRLLRRLRRLLLRRRR".to_string();
    let app = App::new();
    let pos = app
        .input
        .keys()
        .filter(|k| k[2..].to_string() == "A")
        .collect::<Vec<&String>>();
    for start in pos {
        let (steps, pos) = get_steps(&app.input, start.to_string(), inst.clone());
        println!("{:?} - {:?} {}", start, pos, steps);
    }

    // get the lowest common multiple of these numbers
    // HVA - 15871, NMZ
    // LBA - 16409, SJZ
    // FXA - 21251, GNZ
    // GHA - 18023, TNZ
    // PSA - 12643, BNZ
    // AAA - 19099, ZZZ
    let mut m: u128 = 1;
    loop {
        let lcm = 15871 * m;
        if lcm % 16409 == 0
            && lcm % 21251 == 0
            && lcm % 18023 == 0
            && lcm % 12643 == 0
            && lcm % 19099 == 0
        {
            println!("Part B: {:?}", lcm);
            return;
        }
        m += 1;
    }
}

fn get_steps(
    map: &HashMap<String, (String, String)>,
    start: String,
    inst: String,
) -> (usize, String) {
    let mut pos = start.clone();
    let mut steps = 0;
    loop {
        for c in inst.chars() {
            let (left, right) = map.get(&pos).unwrap();
            pos = if c == 'L' { left } else { right }.to_string();
            steps += 1;
            if pos == "BNZ"
                || pos == "TNZ"
                || pos == "ZZZ"
                || pos == "SJZ"
                || pos == "GNZ"
                || pos == "NMZ"
            {
                return (steps, pos);
            }
        }
    }
}
