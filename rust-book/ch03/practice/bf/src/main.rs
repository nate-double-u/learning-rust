// Ch3 - 1 [BF]

// Ask the user for an integer N, and print out the first N Fibonacci
// numbers.

// (Bonus: Have your program do it with a sequential solution first,
// then with a recursive solution)

// Suggestion: Use large int type.

use std::io;

fn main() {
    println!("Fibonacci sequence, to n");
    let n = get_number();
    println!("You chose: {}", n);
    fibonacci_sequential(n)
}

fn get_number() -> u128 {
    println!("Please input a number, N:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u128 = n.trim().parse().expect("Please type a number!");

    n
}

fn fibonacci_sequential(n: u128) {
    let mut previous: u128 = 0;
    let mut current: u128 = 1;
    let mut temp: u128 = 0;

    // cheesing 0 :P
    print!("0 ");

    for number in (1..n) {
        print!("{} ", current);
        temp = current;
        current = previous + current;
        previous = temp;
    }
}
