use rand::seq::IteratorRandom;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::ops::Range;

fn random_guessing(secret_num: u32, range: &Range<u32>) -> u32 {
    let mut unguessed = range.clone().collect::<Vec<_>>();
    let mut guesses = 1;

    loop {
        let guess = *unguessed.clone().iter().choose(&mut rand::thread_rng()).unwrap();
        unguessed.retain(|x| *x != guess);
        if guess == secret_num {
            return guesses
        }
        guesses += 1;
    }

}

fn linear_search(secret_num: u32, range: &Range<u32>) -> u32 {
    secret_num - range.start + 1
}

fn linear_reverse_search(secret_num: u32, range: &Range<u32>) -> u32 {
    range.end - secret_num
}

fn binary_search(secret_num: u32, range: &Range<u32>) -> u32 {
    let mut bounds = (range.start, range.end);

    let mut guesses = 1;
    let mut guess = bounds.0 + (bounds.1 - bounds.0) / 2;

    loop {
        match secret_num.cmp(&guess) {
            Ordering::Equal => return guesses,
            Ordering::Less => {
                bounds.1 = guess;
            }
            Ordering::Greater => {
                bounds.0 = guess;
            }
        };
        guess = bounds.0 + (bounds.1 - bounds.0) / 2;
        guesses += 1;
    }
}

fn main() {
    let mut lower = String::new();
    let mut upper = String::new();

    print!("Lower Bound: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut lower)
        .expect("Could not read user input.");

    print!("Upper Bound: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut upper)
        .expect("Could not read user input.");

    println!();

    let lower: u32 = lower.trim().parse().unwrap();
    let upper: u32 = upper.trim().parse().unwrap();

    let range = lower..upper + 1;

    let mut rng = rand::thread_rng();
    let secret_num = range.clone().choose(&mut rng).unwrap();

    println!("Guessing Number ({} to {})", lower, upper);
    println!("Secret Number: {}", secret_num);
    println!();

    println!("Random Guessing: {} guesses", random_guessing(secret_num, &range));
    println!("Linear Search: {} guesses", linear_search(secret_num, &range));
    println!(
        "Linear Reverse: {} guesses",
        linear_reverse_search(secret_num, &range)
    );
    println!(
        "Binary Search: {} guesses",
        binary_search(secret_num, &range)
    );
}
