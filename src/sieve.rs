fn is_odd(x: u32) -> bool {
    x % 2 == 1
}

fn sqrt_u32(x: u32) -> u32 {
    (x as f32).sqrt() as u32
}

pub fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let lower_bound: u32 = 3;
    let upper_bound: u32 = if is_odd(n) { n + 1 } else { n };

    let num_of_odds: u32 = (upper_bound - (lower_bound - 1)) / 2;

    let mut is_prime_v: Vec<bool> = vec![true; usize::try_from(num_of_odds).unwrap()];

    for i in lower_bound..=sqrt_u32(num_of_odds) {
        if is_prime_v[i as usize] {
            let mut j: u32 = i.pow(2);
            let mut coefficent: u32 = 1;
            while j < num_of_odds {
                is_prime_v[j as usize] = false;
                j = i.pow(2) + coefficent * i;
                coefficent += 1;
            }
        }
    }

    let mut primes: Vec<u32> = vec![2];

    let mut adj: u32 = 3;
    for (pos, is_prime) in is_prime_v.iter().enumerate() {
        if *is_prime {
            primes.push((pos as u32) + adj);
        }

        adj += 1;
    }

    primes
}
