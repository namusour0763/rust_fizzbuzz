// whileとif
fn fizz_buzz1() {
    let mut x = 1;

    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }

        x += 1;
    }
}

// forとrange
fn fizz_buzz2() {
    for x in 1..101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        }
        if x % 3 == 0 {
            println!("Fizz");
        }
        if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

// match
fn fizz_buzz3() {
    for x in 1..=100 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

// マッチガード
fn fizz_buzz4() {
    for x in 1..=100 {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e % 3 == 0 => println!("Fizz"),
            e if e % 5 == 0 => println!("Buzz"),
            e => println!("{}", e),
        }
    }
}

fn main() {
    fizz_buzz1();
    fizz_buzz2();
    fizz_buzz3();
    fizz_buzz4();
}
