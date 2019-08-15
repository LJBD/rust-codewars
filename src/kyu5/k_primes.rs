use std::ops::RangeInclusive;


fn get_primes_up_to(n: i32) -> Vec<i32> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n as usize + 1];
    (0..=1).for_each(|i| is_prime[i] = false);
    for i in get_to_square_range(n) {
        if is_prime[i as usize] {
            for j in (i * i..=n).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    is_prime.iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(p, _)| p as i32)
        .collect()
}

fn is_k_prime(n: i32, k: i32, primes: &Vec<i32>) -> bool {
    let mut counter = 0;
    let mut part_of_n = n;
    primes.iter()
        .for_each(|&i| {
            while part_of_n % i as i32 == 0 {
                part_of_n /= i as i32;
                counter += 1;
                if counter > k {
                    break;
                }
            }
        });
    counter == k
}

fn get_to_square_range(n: i32) -> RangeInclusive<i32> {
    (2..=(n as f32).sqrt().floor() as i32)
}

fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    let primes = get_primes_up_to(nd);
    (start..=nd)
        .filter(|&n| is_k_prime(n, k, &primes))
        .collect()
}

fn puzzle(s: i32) -> i32 {
    let primes = get_primes_up_to(s);
    let three_primes = count_kprimes(3, 0, s);
    let seven_primes = count_kprimes(7, 0, s);
    let mut counter = 0;
    for i in &seven_primes {
        for j in &three_primes {
            for k in &primes {
                if *i + *j + *k == s {
                    counter += 1;
                }
            }
        }
    }
    counter
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

#[test]
fn test_eratosthenes_sieve() {
    assert_eq!(get_primes_up_to(2), vec![2]);
    assert_eq!(get_primes_up_to(10), vec![2, 3, 5, 7]);
    assert_eq!(get_primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
}
