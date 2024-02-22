fn sqrt_u32(x: u32) -> u32 {
    (x as f32).sqrt() as u32
}

pub fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let mut is_prime_v: Vec<bool> = vec![true; usize::try_from(n).unwrap()];

    is_prime_v[0] = false;
    is_prime_v[1] = false;

    for i in 2..=sqrt_u32(n) {
        if is_prime_v[i as usize] {
            let mut j: u32 = i.pow(2);
            let mut coefficent: u32 = 1;
            while j < n {
                is_prime_v[j as usize] = false;
                j = i.pow(2) + coefficent * i;
                coefficent += 1;
            }
        }
    }

    let mut primes: Vec<u32> = Vec::default();

    for (pos, is_prime) in is_prime_v.iter().enumerate() {
        if *is_prime {
            primes.push(pos as u32);
        }
    }

    primes
}
