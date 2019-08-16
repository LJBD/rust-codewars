fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut pairs = vec![];
    let sum: i32 = (0..=m).sum();
    for a in 0..=m {
        for b in 0..=m {
            if a * b == sum - a - b {
                pairs.push((a, b));
            }
        }
    }
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
