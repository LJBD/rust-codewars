use std::ops::RangeInclusive;

fn factors(n: i32) -> Vec<i32> {
    get_to_square_range(n)
        .filter(|&i| n % i == 0)
        .collect()
}

fn is_prime(n: i32) -> bool {
    get_to_square_range(n)
        .filter(|&i| n % i == 0)
        .last().is_none()
}


fn get_to_square_range(n: i32) -> RangeInclusive<i32> {
    (2..=(n as f32).sqrt().floor() as i32)
}

fn prime_factors<'a>(n: i32) -> Vec<i32> {
    factors(n).into_iter()
        .filter(|&i| is_prime(i))
        .collect()
}

fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    (start..=nd)
        .filter(|&n| prime_factors(n).len() == k as usize)
        .collect()
}

fn puzzle(s: i32) -> i32 {
    // your code
    s
}

fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
    assert_eq!(count_kprimes(k, start, nd), exp)
}

#[test]
fn basics_count_kprimes() {
    testing_count_kprimes(5, 1000, 1100, vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
    testing_count_kprimes(12, 100000, 100100, vec![]);
}

fn testing(n: i32, exp: i32) -> () {
    assert_eq!(puzzle(n), exp)
}

#[test]
fn basics_puzzle() {
    testing(100, 0);
    testing(144, 0);
    testing(143, 2);
}
