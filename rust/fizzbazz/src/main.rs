fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    let mut x = 1;

    while x <= 1000 {
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
