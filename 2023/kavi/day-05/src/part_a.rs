use crate::App;

pub fn run() {
    let app = App::new();
    let seeds = app.input.0;
    let maps = app.input.1;

    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut i = seed;
        for map in &maps {
            i = map_i_to_j(i, map);
        }
        if i < lowest {
            lowest = i;
        }
    }

    println!("Part A: {}", lowest);
}

fn map_i_to_j(i: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (dest_start, source_start, source_len) in map {
        if i >= *source_start && i < *source_start + *source_len {
            return *dest_start + (i - source_start);
        }
    }
    i
}
