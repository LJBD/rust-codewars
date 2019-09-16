use std::cmp::min;

fn get_last_digits(number: &str, places: usize) -> i32 {
    number.chars()
        .rev()
        .take(min(number.len(), places))
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn get_last_digit_of_power(n: i32, power: u32) -> i32 {
    let powered = n.pow(power);
    powered - ((powered / 10) * 10)
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }
    let last = get_last_digits(str1, 1);
    match last {
        0 | 1 | 5 | 6 => last,
        4 | 9         => get_last_digit_of_power(last, get_last_digits(str2, 1) as u32 % 2 + 2),
        _             => get_last_digit_of_power(last,
                                                 get_last_digits(str2, 2) as u32 % 4 + 4)
    }
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("2222", "10"), 4);
    assert_eq!(last_digit("10","10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}

#[test]
fn test_getting_last_digit() {
    assert_eq!(get_last_digits("123", 1), 3);
    assert_eq!(get_last_digits("1234", 2), 34);
    assert_eq!(get_last_digits("1", 3), 1);
}

#[test]
fn test_get_last_digit_of_power() {
    assert_eq!(get_last_digit_of_power(1, 3), 1);
    assert_eq!(get_last_digit_of_power(2, 10), 4);
    assert_eq!(get_last_digit_of_power(5, 4), 5);
    assert_eq!(get_last_digit_of_power(10, 2), 0);
}