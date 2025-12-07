use crate::traits::problem::{Neighbours, Score};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use std::fmt::Debug;
use std::vec;

const WORKERS: usize = 10;

// search_exhaustive will search every possible node (by exploring all the neighbours of the seed, etc)
// and return the best one
pub fn search_exhaustive<S: Clone + Debug + Send + Sync + Score<S>>(
    problem: &(impl Neighbours<S> + Sync),
    seed: &S,
) -> S {
    let (tx_ends, rx_ends) = unbounded::<S>();

    let _ = thread::scope(|scope| {
        let mut frontier = vec![];

        // add seed states to frontier
        frontier.push(seed.clone());

        let mut busy = vec![false; WORKERS];

        let (tx_work, rx_work) = unbounded::<S>();
        let (tx_mids, rx_mids) = unbounded::<S>();
        let (tx_busy, rx_busy) = unbounded::<(usize, bool)>();

        // master
        scope.spawn(move |_| {
            loop {
                // send as much work as possible to the workers
                while frontier.len() > 0 && busy.iter().any(|&x| x == false) {
                    match tx_work.send(frontier.pop().unwrap()) {
                        Err(_) => break,
                        _ => {}
                    }
                    // mark the thread that took the work as busy
                    match rx_busy.recv() {
                        Ok((i, b)) => {
                            busy[i] = b;
                        }
                        Err(_) => break,
                    }
                }
                // else block until we get all the mids
                while !rx_mids.is_empty() {
                    match rx_mids.recv() {
                        Ok(mid) => {
                            frontier.push(mid);
                        }
                        Err(_) => break,
                    }
                }
                // else block until we get all the busy updates
                while !rx_busy.is_empty() {
                    match rx_busy.recv() {
                        Ok((i, b)) => {
                            busy[i] = b;
                        }
                        Err(_) => break,
                    }
                }

                // there's no more work to do and no threads working on anything
                if frontier.len() == 0 && busy.iter().all(|&x| x == false) {
                    break;
                }
                //println!("frontier: {}, busy: {:?}", frontier.len(), busy);
            }
            drop(tx_work);
        });

        // workers
        for i in 0..WORKERS {
            let my_rx_work = rx_work.clone();
            let my_tx_mids = tx_mids.clone();
            let my_tx_busy = tx_busy.clone();
            let my_tx_ends = tx_ends.clone();
            scope.spawn(move |_| {
                loop {
                    //                    println!("worker {} waiting for work", i);
                    match my_rx_work.recv() {
                        Ok(rx) => {
                            //                                println!("Thread {} got message {:?}", i, rx);
                            //                            println!("worker {} got work {:?}", i, rx);
                            let _ = my_tx_busy.send((i, true));
                            //                            println!("worker {} sent busy", i);
                            let (ends, mids) = search_recursive(problem, &rx, 0);
                            //                            println!("worker {} did work", i);
                            for mid in mids {
                                my_tx_mids.send(mid).unwrap();
                            }
                            for end in ends {
                                my_tx_ends.send(end).unwrap();
                            }
                            let _ = my_tx_busy.send((i, false));
                        }
                        Err(_) => {
                            //                            println!("worker {} ending", i);
                            break;
                        }
                    }
                }
            });
        }
    });

    let mut best_state = seed.clone();
    let mut best_score = 0;
    while !rx_ends.is_empty() {
        match rx_ends.recv() {
            Ok(end) => {
                let score = end.score();
                if score > best_score {
                    best_score = score;
                    best_state = end;
                }
            }
            Err(_) => break,
        }
    }

    best_state
}

fn search_recursive<S: Clone + Debug>(
    problem: &impl Neighbours<S>,
    seed: &S,
    depth: usize,
) -> (Vec<S>, Vec<S>) {
    let neighbours = problem.neighbours(seed);
    if neighbours.len() == 0 {
        return (vec![seed.clone()], vec![]);
    }
    if depth > 300 {
        return (vec![], vec![seed.clone()]);
    }
    let mut ret_ends = vec![];
    let mut ret_mids = vec![];
    let num_neighbours = neighbours.len();
    for neighbour in neighbours {
        let (ends, mids) = search_recursive(problem, &neighbour, depth + num_neighbours);
        ret_ends.extend(ends);
        ret_mids.extend(mids);
    }
    return (ret_ends, ret_mids);
}
