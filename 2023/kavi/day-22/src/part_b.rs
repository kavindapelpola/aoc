use crate::{
    part_a::{get_bricks, settle_bricks},
    App,
};

pub fn run() {
    let app = App::new();

    let bricks = get_bricks(&app.input);

    let (_, supported_matrix) = settle_bricks(&bricks);

    println!("{:?}", how_many_will_fall(&supported_matrix, 0));

    //    println!("Part B: {}", total);
}

fn how_many_will_fall(
    supported_matrix: &Vec<(usize, Vec<usize>)>,
    i: usize,
    supporting: &Vec<usize>,
) -> usize {
    let mut count = 0;

    println!("{:?} {:?}", i, supported_matrix);

    let supporting = supported_matrix
        .iter()
        .filter(|(_, v)| v.contains() && v.len() == 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut total = supporting.len();
    for i in supporting {
        total += how_many_will_fall(supported_matrix, supporting);
    }
    println!("{:?}", total);
    total
    // for (i, v) in supported_matrix {
    //     if v.contains(&index) && v.len() == 1 {
    //         count += 1;
    //     }
    // }
    // count
}
