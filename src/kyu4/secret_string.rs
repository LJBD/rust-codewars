fn contains_char(c: char, subject: &String) -> bool {
    subject.chars().any(|x| c == x )
}

fn index_of(c: char, chars: &Vec<char>) -> usize {
    chars.iter()
        .enumerate()
        .filter(|(_, &x)| x == c)
        .map(|(index, _)| index)
        .next()
        .unwrap()
}

fn take_first_omitting(chars_matrix: &Vec<Vec<char>>, omit: &String) -> Vec<char> {
    chars_matrix.iter()
        .map(|chars| chars[0])
        .filter(|&c| !contains_char(c, omit))
        .collect()
}

fn take_all_omitting(triplets: &Vec<[char; 3]>, omit: &String) -> Vec<Vec<char>> {
    triplets.iter()
        .map(|&chars| chars.to_vec().iter()
            .filter(|&&c| !contains_char(c, omit))
            .map(|c| c.to_owned())
            .collect::<Vec<char>>()
        )
        .filter(|v| !v.is_empty())
        .collect()
}

fn positions_of(c: char, chars_matrix: &Vec<Vec<char>>) -> Vec<usize> {
    chars_matrix.iter()
        .filter(|&chars| chars.contains(&c))
        .map(|chars| index_of(c, chars))
        .collect()
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut secret = "".to_string();
    loop {
        let chars_matrix = take_all_omitting(&triplets, &secret);
        println!("ALL: {:?}", chars_matrix);
        if chars_matrix.is_empty() { break; }
        take_first_omitting(&chars_matrix, &secret).iter()
            .filter(|&&c| {
                let positions = positions_of(c, &chars_matrix);
                println!("Found {} at {:?}", c, positions);
                positions.is_empty() || positions.iter().all(|&pos| pos == 0)
            })
            .take(1)
            .for_each(|&c| secret.push(c))
    }
    secret
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
    assert_eq!(recover_secret(vec![
        ['s', 'u', 'p']
    ]), "sup");
    assert_eq!(recover_secret(vec![
        ['s', 'u', 'p'],
        ['o', 'u', 'p']
    ]), "soup")
}
