struct Fibonacci {
    curr: u128,
    next: u128
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    for(idx, val) in Fibonacci::default().take(20).enumerate() {
        println!("{idx: >3}: {: >10}", val);
    }

    let first_10_fibs: Vec<u128> = Fibonacci::default().take(10).collect();
    println!("{:?}", first_10_fibs);

    let result: u128 = Fibonacci::default().take(10).map(|el| el * 2).sum();
    println!("{result}");
}
