fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 52; // Hard-coded secret number
    let mut guess;
    let mut attempts = 0;

    loop {
        guess = 0 + attempts; // Simulating user input
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The number was {}.", secret);
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }

    println!("It took {} guesses.", attempts);
}
