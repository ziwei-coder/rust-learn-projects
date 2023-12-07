use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand_gen(max: usize) -> usize {
    RG.lock().unwrap().next_val(max)
}

struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    fn new(curr: usize) -> Self {
        Self {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }

    fn next_val(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rands() {
        let mut rg = RandGen::new(12);

        for _ in 0..10 {
            println!("--{}", rg.next_val(5))
        }
    }
}
