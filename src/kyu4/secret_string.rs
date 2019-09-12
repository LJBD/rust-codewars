use std::collections::HashMap;

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut secret = "".to_string();
    let mut trip = triplets;
    let letters = get_letters_map(&triplets);
    secret
}

fn get_letters_map(triplets: &Vec<[char; 3]>) -> HashMap<char, [u8; 3]> {
    let mut letters: HashMap<char, [u8; 3]> = HashMap::new();
    triplets.iter()
        .for_each(|&chars| {
            chars.iter().enumerate().for_each(|(i, &c)| {
                if !letters.contains_key(&c) {
                    letters.insert(c, [0; 3]);
                } else {
                    let mut v = letters.get_mut(&c).unwrap();
                    *v[i] += 1;
                }
            })
        });
    letters
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn example_test() {
    assert_eq!(recover_secret(vec![
        ['t','u','p'],
        ['w','h','i'],
        ['t','s','u'],
        ['a','t','s'],
        ['h','a','p'],
        ['t','i','s'],
        ['w','h','s']])
               , "whatisup");
}