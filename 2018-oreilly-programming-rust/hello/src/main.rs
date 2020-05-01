/// gcd takes n and m, and returns the greatest common divisor
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);  // panics if either value is 0 and exits the program
    while m != 0 {              // runs as long as m isn't 0
        if m < n {              // checks if m is less than n
            let t = m;          // defines t as m, it derives u64 type
            m = n;              // set m to value of n
            n = t;              // sets n to value of t
        }                       // t goes out of scope here
        m = m % n;              // sets m as result of m mod n
    }
    n                           // returns n, note the lack of a trailing ;
}

#[test]                                     // attribute that denotes a unit test
fn test_gcd() {                             // function signature
    assert_eq!(gcd(14,15), 1);              // assert_eq! compares left and right value and panics
                                            // on failure, used mostly within tests
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                   3 * 11);
}



/// Entry point of a rust program
fn main() {
    // Prints hello world using println! macro.
    // println! prints to stdout and appends a new line character
    println!("Hello, world!");
}
