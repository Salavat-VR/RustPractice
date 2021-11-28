fn main() {
    assert_eq!(nth(5), 13);
}

pub fn nth(n: u32) -> u32 {
    let mut p = 3;
    let mut primes = vec![2];
    while n >= primes.len() as u32 {
        if !primes.iter().any(|pn| p % pn == 0) {
            primes.push(p);
        }
        p += 1;
    }
    primes.pop().unwrap()
}
