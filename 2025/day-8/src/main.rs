use std::{collections::HashMap, fs, vec};

type Point = (u64, u64, u64);

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    //    part1(&input);
    part2(&input);
}

fn distance(p1: &Point, p2: &Point) -> u64 {
    return p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2) + p1.2.abs_diff(p2.2).pow(2);
}

fn merge_circuits(
    circuits: &mut Vec<Vec<usize>>,
    junction_circuit: &mut HashMap<usize, usize>,
    from_circuit: usize,
    to_circuit: usize,
) {
    junction_circuit
        .iter_mut()
        .filter(|(_, v)| **v == from_circuit)
        .for_each(|(_, v)| *v = to_circuit);
    let from_junctions = circuits[from_circuit].clone();
    circuits[to_circuit].append(&mut from_junctions.clone());
    circuits[from_circuit].clear();
}

fn part1(input: &str) {
    let junctions = input
        .lines()
        .map(|l| {
            let points = l
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            return (points[0], points[1], points[2]);
        })
        .collect::<Vec<Point>>();
    let mut distances = vec![];
    for i in 0..junctions.len() {
        for j in 0..i {
            distances.push((i, j, distance(&junctions[i], &junctions[j])));
        }
    }
    distances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut junction_circuit: HashMap<usize, usize> = HashMap::new();
    for _ in 0..1000 {
        let closest = distances.pop();
        if closest.is_none() {
            break;
        }
        let (i, j, _) = closest.unwrap();
        if junction_circuit.contains_key(&i) && junction_circuit.contains_key(&j) {
            if junction_circuit[&i] != junction_circuit[&j] {
                // merge circuits (from j to i)
                let from_circuit = junction_circuit[&j];
                let to_circuit = junction_circuit[&i];
                merge_circuits(
                    &mut circuits,
                    &mut junction_circuit,
                    from_circuit,
                    to_circuit,
                );
            }
            // if they are the same do nothing
            continue;
        }
        if junction_circuit.contains_key(&i) && !junction_circuit.contains_key(&j) {
            // i is in a circuit so add j to that circuit and map
            circuits[junction_circuit[&i]].push(j);
            junction_circuit.insert(j, junction_circuit[&i]);
            continue;
        }
        if junction_circuit.contains_key(&j) && !junction_circuit.contains_key(&i) {
            // j is in a circuit
            circuits[junction_circuit[&j]].push(i);
            junction_circuit.insert(i, junction_circuit[&j]);
            continue;
        }
        // neither is in a circuit, so create a new circuit and map
        circuits.push(vec![i, j]);
        junction_circuit.insert(i, circuits.len() - 1);
        junction_circuit.insert(j, circuits.len() - 1);
    }
    let mut sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    sizes.sort();
    sizes.reverse();
    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}

fn part2(input: &str) {
    let junctions = input
        .lines()
        .map(|l| {
            let points = l
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            return (points[0], points[1], points[2]);
        })
        .collect::<Vec<Point>>();
    let mut distances = vec![];
    for i in 0..junctions.len() {
        for j in 0..i {
            distances.push((i, j, distance(&junctions[i], &junctions[j])));
        }
    }
    distances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut junction_circuit: HashMap<usize, usize> = HashMap::new();
    loop {
        let closest = distances.pop();
        if closest.is_none() {
            break;
        }
        println!(
            "{} {} - {}",
            junctions[closest.unwrap().0].0,
            junctions[closest.unwrap().1].0,
            junctions[closest.unwrap().0].0 * junctions[closest.unwrap().1].0
        );
        if circuits.len() > 1 && circuits.iter().filter(|c| c.len() > 0).count() == 1 {
            break;
        }
        let (i, j, _) = closest.unwrap();
        if junction_circuit.contains_key(&i) && junction_circuit.contains_key(&j) {
            if junction_circuit[&i] != junction_circuit[&j] {
                // merge circuits (from j to i)
                let from_circuit = junction_circuit[&j];
                let to_circuit = junction_circuit[&i];
                merge_circuits(
                    &mut circuits,
                    &mut junction_circuit,
                    from_circuit,
                    to_circuit,
                );
            }
            // if they are the same do nothing
            continue;
        }
        if junction_circuit.contains_key(&i) && !junction_circuit.contains_key(&j) {
            // i is in a circuit so add j to that circuit and map
            circuits[junction_circuit[&i]].push(j);
            junction_circuit.insert(j, junction_circuit[&i]);
            continue;
        }
        if junction_circuit.contains_key(&j) && !junction_circuit.contains_key(&i) {
            // j is in a circuit
            circuits[junction_circuit[&j]].push(i);
            junction_circuit.insert(i, junction_circuit[&j]);
            continue;
        }
        // neither is in a circuit, so create a new circuit and map
        circuits.push(vec![i, j]);
        junction_circuit.insert(i, circuits.len() - 1);
        junction_circuit.insert(j, circuits.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_circuits_works() {
        let mut circuits = vec![vec![0, 1], vec![2, 3]];
        let mut junction_circuit: HashMap<usize, usize> =
            HashMap::from([(0, 0), (1, 0), (2, 1), (3, 1)]);
        println!("{:?}", junction_circuit);
        println!("{:?}", circuits);
        merge_circuits(&mut circuits, &mut junction_circuit, 1, 0);
        assert_eq!(circuits, vec![vec![0, 1, 2, 3], vec![]]);
        assert_eq!(
            junction_circuit,
            HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0)])
        );
    }
}
