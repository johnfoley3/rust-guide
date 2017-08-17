extern crate rand;

use rand::Rng; // trait must be in scope to use the methods implemented

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
}
