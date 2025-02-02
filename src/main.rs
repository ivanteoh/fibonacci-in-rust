use std::io;

fn main() {

    loop {
        println!("Please input a number to generate the nth Fibonacci number from zero onward");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let mut value = [0, 1];

        let mut result = 0;

        if guess == 1 {
            result = 1;
        }

        for _number in 1..guess {
            result = value[0] + value[1];
            value[0] = value[1];
            value[1] = result;
        }

        println!("The Fibonacci number: {}", result);
    }
}
