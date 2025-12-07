use std::{collections::HashMap, sync::Mutex};

use crate::{part_a::arrangements, App};
use itertools::Itertools;
use rayon::prelude::*;

pub fn run() {
    let app = App::new();
    let sum = Mutex::new(0);

    app.input.par_iter().for_each(|i| {
        let mut cache = HashMap::new();
        let s = (0..5).map(|_| i.0.clone()).join("?");
        let v = (0..5).flat_map(|_| i.1.clone()).collect::<Vec<_>>();

        let tot = arrangements(s.clone(), v.clone(), None, &mut cache);
        let mut sum = sum.lock().unwrap();
        *sum += tot;
    });

    println!("Part B: {}", sum.lock().unwrap());
}
