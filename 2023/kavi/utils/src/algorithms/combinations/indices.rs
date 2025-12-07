use itertools::Itertools;

// returns a vector of index combinations
// the input vec order is the is the order of the indices
// if the input vec is empty, then the indices will be none (since 0 is an actual index)
pub fn index_combinations<T>(vec_of_vecs: &Vec<Vec<T>>) -> Vec<Vec<Option<usize>>> {
    // cant do anything if the input is empty
    if vec_of_vecs.len() == 0 {
        return vec![];
    }

    // initialise the answer to the a vec of all the indices of the first vector
    // if the first vector is empty, then the indices of the first vector is None
    let mut ans: Vec<Vec<Option<usize>>> = if vec_of_vecs[0].len() == 0 {
        vec![vec![None]]
    } else {
        (0..vec_of_vecs[0].len())
            .map(|i| vec![Some(i)])
            .collect::<Vec<_>>()
    };

    // use the product of this first vector with the rest of the vectors to get the answer
    for i in 1..vec_of_vecs.len() {
        if vec_of_vecs[i].len() == 0 {
            ans = ans
                .iter()
                .map(|v| {
                    let mut v = v.clone();
                    v.push(None);
                    v
                })
                .collect::<Vec<_>>();
            continue;
        }
        ans = ans
            .iter()
            .cartesian_product((0..vec_of_vecs[i].len()).collect::<Vec<usize>>())
            .map(|(v, i)| {
                let mut v = v.clone();
                v.push(Some(i));
                v
            })
            .collect::<Vec<_>>();
    }
    ans
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn index_combinations_returns_combinations_for_happy_path() {
        let index_len = vec![vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]];
        let ans = index_combinations(&index_len);
        let expected = vec![
            vec![Some(0), Some(0), Some(0)],
            vec![Some(0), Some(0), Some(1)],
            vec![Some(0), Some(0), Some(2)],
            vec![Some(0), Some(0), Some(3)],
            vec![Some(0), Some(1), Some(0)],
            vec![Some(0), Some(1), Some(1)],
            vec![Some(0), Some(1), Some(2)],
            vec![Some(0), Some(1), Some(3)],
            vec![Some(0), Some(2), Some(0)],
            vec![Some(0), Some(2), Some(1)],
            vec![Some(0), Some(2), Some(2)],
            vec![Some(0), Some(2), Some(3)],
            vec![Some(1), Some(0), Some(0)],
            vec![Some(1), Some(0), Some(1)],
            vec![Some(1), Some(0), Some(2)],
            vec![Some(1), Some(0), Some(3)],
            vec![Some(1), Some(1), Some(0)],
            vec![Some(1), Some(1), Some(1)],
            vec![Some(1), Some(1), Some(2)],
            vec![Some(1), Some(1), Some(3)],
            vec![Some(1), Some(2), Some(0)],
            vec![Some(1), Some(2), Some(1)],
            vec![Some(1), Some(2), Some(2)],
            vec![Some(1), Some(2), Some(3)],
        ];
        assert_eq!(ans, expected);
    }

    #[test]
    fn index_combinations_returns_combinations_for_empty_at_end() {
        let index_len = vec![vec![0, 1], vec![0, 1, 2], vec![]];
        let ans = index_combinations(&index_len);
        let expected = vec![
            vec![Some(0), Some(0), None],
            vec![Some(0), Some(1), None],
            vec![Some(0), Some(2), None],
            vec![Some(1), Some(0), None],
            vec![Some(1), Some(1), None],
            vec![Some(1), Some(2), None],
        ];
        assert_eq!(ans, expected);
    }

    #[test]
    fn index_combinations_returns_combinations_for_empty_at_start() {
        let index_len = vec![vec![], vec![0, 1], vec![0, 1, 2]];
        let ans = index_combinations(&index_len);
        let expected = vec![
            vec![None, Some(0), Some(0)],
            vec![None, Some(0), Some(1)],
            vec![None, Some(0), Some(2)],
            vec![None, Some(1), Some(0)],
            vec![None, Some(1), Some(1)],
            vec![None, Some(1), Some(2)],
        ];
        assert_eq!(ans, expected);
    }

    #[test]
    fn index_combinations_returns_empty_for_empty_input() {
        let index_len: Vec<Vec<u32>> = vec![];
        let ans = index_combinations(&index_len);
        let expected: Vec<Vec<Option<usize>>> = vec![];
        assert_eq!(ans, expected);
    }

    #[test]
    fn index_combinations_returns_single_for_single_vec_input() {
        let index_len: Vec<Vec<u32>> = vec![vec![0, 1]];
        let ans = index_combinations(&index_len);
        let expected: Vec<Vec<Option<usize>>> = vec![vec![Some(0)], vec![Some(1)]];
        assert_eq!(ans, expected);
    }
}
