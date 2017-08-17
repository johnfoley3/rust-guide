use std::cmp::Ordering;

fn main() {
    let secret_number = 2;
    let guess = 3;

    match guess.cmp(&secret_number) { // cmp returns an enumerable with variants of type Ordering
        Ordering::Less => println!("Too small!"), // each variant is an arm of the match construct
        Ordering::Greater => println!("Too big!"), // each arm has a pattern, and a execution block
        Ordering::Equal => println!("You win!"),
    }
}
