#![allow(dead_code)]

use std::time::Instant;

fn main() {
    let start = Instant::now();

    problem39::main();

    println!("\n{:?} elapsed", start.elapsed());
}

/// https://projecteuler.net/problem=10
mod problem10 {
    const N: usize = 2_000_000;

    pub fn main() {
        let mut primes = vec![true; N];
        primes[0] = false;
        primes[1] = false;

        for i in 2..N {
            if primes[i] {
                let mut j = 2;
                while i * j < N {
                    primes[i * j] = false;
                    j += 1;
                }
            }
        }

        let sum = primes.iter().enumerate().fold(
            0,
            |acc, (i, &is_prime)| if is_prime { acc + i } else { acc },
        );

        println!("{}", sum);
    }
}

/// https://projecteuler.net/problem=14
mod problem14 {
    use std::collections::HashMap;

    const N: usize = 1_000_000;

    pub fn main() {
        let mut longest_chain = 0;
        let mut number_with_longest_chain = 1;

        let mut chain_lengths = HashMap::new();

        for i in 1..N {
            let mut chain_length = 1;

            let mut k = i;
            while k != 1 {
                k = compute_collatz(k);
                chain_length += 1;

                if let Some(&l) = chain_lengths.get(&k) {
                    // -1 otherwise the current term is counted twice
                    chain_length += l - 1;
                    break;
                }
            }

            chain_lengths.insert(i, chain_length);

            if chain_length > longest_chain {
                longest_chain = chain_length;
                number_with_longest_chain = i;
            }
        }

        println!(
            "{} is the number below {} with longest collatz sequence ({} terms)",
            number_with_longest_chain, N, longest_chain
        );
    }

    #[inline]
    const fn compute_collatz(n: usize) -> usize {
        if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        }
    }
}

/// https://projecteuler.net/problem=22
mod problem22 {
    const NAMES: &str = include_str!("../assets/0022_names.txt");

    pub fn main() {
        let mut names = NAMES
            .to_string()
            .split(",")
            .map(|s| s[1..s.len() - 1].to_string())
            .collect::<Vec<_>>();

        names.sort();

        let sum = names
            .iter()
            .enumerate()
            .map(|(i, n)| (i + 1) as u128 * n.chars().fold(0, |acc, c| acc + (c as u128 - 64)))
            .sum::<u128>();

        println!("{}", sum);
    }
}

/// https://projecteuler.net/problem=22
mod problem39 {

    const N: usize = 1000;

    pub fn main() {
        let mut max_solutions = 0;
        let mut max_solutions_p = 0;
        let mut solutions = Vec::new();

        for p in 1..=N {
            let mut n_solutions_p = 0;
            let mut solutions_p = Vec::new();

            // a is the smallest of the three sides of the triangle so
            // it has to be smaller than a third of the perimeter
            for a in 1..p / 3 {
                for b in 1..p - a {
                    let c = p - a - b;

                    if a <= b && b <= c && a + b + c == p && a * a + b * b == c * c {
                        n_solutions_p += 1;
                        solutions_p.push((a, b, c));
                    }
                }
            }

            if n_solutions_p > max_solutions {
                max_solutions = n_solutions_p;
                max_solutions_p = p;
                solutions = solutions_p;
            }
        }

        println!(
            "max solutions {} for p = {}\n solutions are: {:?}",
            max_solutions, max_solutions_p, solutions
        );

        assert!(solutions.iter().all(|(a, b, c)| a <= b && b <= c));

        // this could be optimized more but it works so...
    }
}
