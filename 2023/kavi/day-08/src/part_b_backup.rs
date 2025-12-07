use crate::App;

pub fn run() {
    let inst = "LRLRRRLRLLRRLRLRRRLRLRRLRRLLRLRRLRRLRRRLRRRLRLRRRLRLRRLRRLLRLRLLLLLRLRLRRLLRRRLLLRLLLRRLLLLLRLLLRLRRLRRLRRRLRRRLRRLRRLRRRLRLRLRRLRLRLRLRRLRRRLLRLLRRLRLRRRLRLRRRLRLRRRLRRRLRRLRLLLLRLRRRLRLRRLRLRRLRRLRRLLRRRLLLLLLRLRRRLRRLLRRRLRRLLLRLRLRLRRRLRRLRLRRRLRRLRRRLLRRLRRLLLRRRR".to_string();
    let app = App::new();
    let mut pos = app
        .input
        .keys()
        .filter(|k| k[2..].to_string() == "A")
        .collect::<Vec<&String>>();
    let mut record = vec![];
    record.push(pos.clone());
    let len = pos.len();
    let mut steps = 0;
    for c in inst.chars() {
        for p in &mut pos {
            let (left, right) = app.input.get(&p.clone()).unwrap();
            if c == 'L' {
                *p = left;
            } else {
                *p = right;
            };
        }
        steps += 1;
        if pos.iter().filter(|k| k[2..].to_string() == "Z").count() == len {
            println!("Part B: {:?}", steps);
            return;
        }
        if record.contains(&pos) {
            println!("{:?}", record);
            println!("Part B: {:?}", steps);
            return;
        }
        record.push(pos.clone());
    }
    for r in record {
        println!("{:?}", r);
    }
}
