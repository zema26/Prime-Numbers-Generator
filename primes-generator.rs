/// Prime number generator using the Sieve of Eratosthenes algorithm.
/// This struct implements the Iterator trait to yield primes on demand.
pub struct PrimeGenerator {
    sieve: Vec<bool>,
    current_index: usize,
    limit: usize,
    yielded_two: bool,
}

impl PrimeGenerator {
    /// Creates a new prime generator that will yield primes up to the given limit.
    pub fn new(limit: usize) -> Self {
        if limit < 2 {
            return PrimeGenerator {
                sieve: vec![],
                current_index: 0,
                limit,
                yielded_two: true,
            };
        }

        // Create sieve for odd numbers only (3, 5, 7, ...)
        // sieve[i] represents whether (2*i + 3) is prime
        let sieve_size = if limit >= 3 { (limit - 1) / 2 } else { 0 };
        let mut sieve = vec![true; sieve_size];

        let mut i = 0;
        let mut index_square = 3; // index for p^2 where p = 2*0 + 3 = 3

        // Sieve of Eratosthenes: mark composites
        while index_square < sieve_size {
            if sieve[i] {
                let prime = 2 * i + 3;
                
                // Mark multiples of this prime as composite
                for j in (index_square..sieve_size).step_by(prime) {
                    sieve[j] = false;
                }
            }

            i += 1;
            // Calculate index for the square of the next potential prime
            index_square = 2 * i * (i + 3) + 3;
        }

        PrimeGenerator {
            sieve,
            current_index: 0,
            limit,
            yielded_two: false,
        }
    }

    /// Returns all primes up to the limit as a vector
    pub fn collect_all(&mut self) -> Vec<usize> {
        self.collect()
    }
}

impl Iterator for PrimeGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // First yield 2 (the only even prime)
        if !self.yielded_two && self.limit >= 2 {
            self.yielded_two = true;
            return Some(2);
        }

        // Then yield odd primes from the sieve
        while self.current_index < self.sieve.len() {
            let i = self.current_index;
            self.current_index += 1;

            if self.sieve[i] {
                let prime = 2 * i + 3;
                if prime <= self.limit {
                    return Some(prime);
                } else {
                    return None;
                }
            }
        }

        None
    }
}

fn main() {
    println!("Prime Generator Demo");
    println!("====================\n");

    // Example 1: Generate primes up to 1000 (matching original code)
    println!("Primes up to 1000:");
    let generator = PrimeGenerator::new(1000);
    let primes: Vec<usize> = generator.collect();
    
    // Print 10 primes per line
    for (i, prime) in primes.iter().enumerate() {
        print!(" {}", prime);
        if (i + 1) % 10 == 0 {
            println!();
        }
    }
    println!("\n\nNumber of primes: {}\n", primes.len());

    // Example 2: Generate first 20 primes
    println!("First 20 primes:");
    let generator = PrimeGenerator::new(100);
    let first_20: Vec<usize> = generator.take(20).collect();
    println!("{:?}", first_20);
    println!();

    // Example 3: Find primes in a specific range
    println!("Primes between 100 and 200:");
    let primes_in_range: Vec<usize> = PrimeGenerator::new(200)
        .filter(|&p| p >= 100)
        .collect();
    println!("{:?}", primes_in_range);
    println!();

    // Example 4: Sum of primes up to 100
    let sum: usize = PrimeGenerator::new(100).sum();
    println!("Sum of all primes up to 100: {}", sum);
}
