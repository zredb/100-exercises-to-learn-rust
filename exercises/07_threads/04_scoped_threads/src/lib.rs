// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::{thread, vec};

pub fn sum(v: Vec<i32>) -> i32 {

    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    // Convert slices to owned vectors for the threads
    let left_vec = left.to_vec();
    let right_vec = right.to_vec();

    let total_sum = thread::scope(|s|{
        let ls=s.spawn(move||{
            left_vec.iter().sum::<i32>()
        });
        let rs=s.spawn(move||{
            right_vec.iter().sum::<i32>()
        });
        ls.join().unwrap() + rs.join().unwrap()
    });

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
