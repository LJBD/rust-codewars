use std::cmp::min;

fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut solution = vec![];
    let mut total = n.pow(2);
    let mut i = n - 1;
    loop {
        i = min((total as f64).sqrt().floor() as i64, i);
        println!("Current i: {}", i);
        if i.pow(2) == total {
            solution.push(i);
            return Some(solution)
        } else {
            total -= i.pow(2);
            if total < 5 {
                break;
            }
            i -= 1;
            solution.push(i);
        }
    }
    None
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}
