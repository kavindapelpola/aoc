use crate::{part_a::hash, App};

pub fn run() {
    let app = App::new();
    let mut boxes = vec![Vec::<(String, u64)>::new(); 256];
    for operation in app.input {
        do_op(operation, &mut boxes);
    }
    let mut tot = 0;
    for (box_num, lenses) in boxes.iter().enumerate() {
        tot += focussing_power(box_num, lenses.clone());
    }
    println!("Part B: {}", tot);
}

fn do_op(op: String, boxes: &mut Vec<Vec<(String, u64)>>) {
    if op.contains("=") {
        let parts = op.split("=").collect::<Vec<&str>>();
        let label = parts[0].to_string();
        let focal_len: u64 = parts[1].parse().unwrap();
        let box_index = hash(label.clone());
        if let Some(item) = boxes[box_index as usize].iter_mut().find(|x| x.0 == label) {
            item.1 = focal_len;
        } else {
            boxes[box_index as usize].push((label, focal_len));
        }
        return;
    }
    // its a minus
    let label = op[..op.len() - 1].to_string();
    let box_index = hash(label.clone());
    boxes[box_index as usize].retain(|x| x.0 != label);
}

fn focussing_power(box_num: usize, lenses: Vec<(String, u64)>) -> u64 {
    let mut fp = vec![];
    for (slot_index, l) in lenses.iter().enumerate() {
        fp.push((box_num + 1) as u64 * (slot_index + 1) as u64 * l.1);
    }
    fp.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut boxes = vec![Vec::<(String, u64)>::new(); 256];
        do_op("rn=1".to_string(), &mut boxes);
        println!("{:?}", boxes);
        do_op("cm-".to_string(), &mut boxes);
        println!("{:?}", boxes);
        do_op("qp=3".to_string(), &mut boxes);
        println!("{:?}", boxes);
        do_op("cm=2".to_string(), &mut boxes);
        println!("{:?}", boxes);
        do_op("qp-".to_string(), &mut boxes);
        println!("{:?}", boxes);
        assert!(false)
    }
}
