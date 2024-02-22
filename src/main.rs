mod helpers;
mod sieve;

use crate::sieve::sieve_of_eratosthenes;

fn main() {
    let n: u32 = 30;
    let primes: Vec<u32> = sieve_of_eratosthenes(n);
    println!("{:?}", primes);
}
