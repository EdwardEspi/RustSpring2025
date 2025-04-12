fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 7, 15, 9, 22, 5, 30, 18, 3, 25];
    
    // For loop to analyze numbers
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            let parity = if is_even(num) { "Even" } else { "Odd" };
            println!("{} is {}", num, parity);
        }
    }
    
    // While loop to calculate sum
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);
    
    // Finding the largest number using a loop
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
