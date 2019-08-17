fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut pairs = vec![];
    let sum: u64 = (1..=m as u64).sum();
    for a in (m as u64 / 2..=m as u64).rev() {
        let remainder = (sum - a) % a;
        let b = (sum - a) / a;
        if remainder <= m as u64 && b <= m as u64 && b == remainder {
            pairs.push((b as i32, a as i32));
            pairs.push((a as i32, b as i32));
        }
    }
    pairs.sort_by_key(|(a, _)| *a);
    pairs
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {

    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);

}
