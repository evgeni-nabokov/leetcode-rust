use super::*;
use crate::challenge_2020_10::recent_counter::RecentCounter;

#[test]
fn max_distance_test() {
    let test_cases = vec![
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5],
                vec![1, 2, 3]
            ],
            4
        ),
        (
            vec![
                vec![-1, 2, 3],
                vec![4, 5],
                vec![-1, 2, 3]
            ],
            6
        ),
        (
            vec![
                vec![-8, -7, -7, -5, 1, 1, 3, 4],
                vec![-2],
                vec![-10, -10, -7, 0, 1, 3],
                vec![2]
            ],
            14
        ),
        (
            vec![
                vec![1, 2, 3, 4, 5, 6],
                vec![3, 4],
                vec![4, 5],
            ],
            4
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_distance(case.0), case.1);
    }
}

#[test]
fn recent_counter_test() {
    let test_cases = vec![
        (1, 1),
        (100, 2),
        (3001, 3),
        (3002, 3),
    ];

    let mut obj = RecentCounter::new();
    for case in test_cases {
        assert_eq!(obj.ping(case.0), case.1);
    }
}

#[test]
fn find_pairs_test() {
    let test_cases = vec![
        (vec![1, 1, 1, 1, 1], 0, 1),
        (vec![3, 1, 4, 1, 5], 2, 2),
        (vec![1, 2, 3, 4, 5], 1, 4),
        (vec![1, 3, 1, 5, 4], 0, 1),
        (vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3, 2),
        (vec![-1, -2, -3], 1, 2),
    ];

    for case in test_cases {
        assert_eq!(Solution::find_pairs(case.0, case.1), case.2);
    }
}