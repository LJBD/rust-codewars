fn solution(num: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum = i + sum;
        }
    }
    sum
}

#[test]
fn sample_tests() {
    // assertion(expected, input);
    assertion(  23,   10);
    assertion(  33,   11);
    assertion( 225,   33);
    assertion(   8,    6);
    assertion(3420,  123);
    assertion( 543,   50);
    assertion(   0,    0);
    assertion(   0, -203);
    assertion(25719750, 10500);
}

fn assertion(expected : i32, input : i32) {
    let actual = solution(input);

    assert_eq!(expected, actual, "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input);
}