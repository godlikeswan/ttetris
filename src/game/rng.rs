use std::time::SystemTime;

pub struct Rng {
    pub t: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut t = seed;
        if t == 0 {
            t += 2147483646;
        }
        t %= 2147483647;
        Rng { t }
    }

    pub fn make() -> Self {
        Self::new(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as _,
        )
    }

    pub fn next(&mut self) -> u64 {
        self.t = 16807 * self.t % 2147483647;
        self.t
    }

    pub fn next_float(&mut self) -> f64 {
        (self.next() as f64 - 1.0) / 2147483646.0
    }

    pub fn shuffle<T>(&mut self, arr_like: &mut [T]) {
        let len = arr_like.len();
        if len == 0 {
            return;
        }
        for i in (1..=(len - 1)).rev() {
            let r = (self.next_float() * (i + 1) as f64) as usize;
            arr_like.swap(i, r);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numbers() {
        let mut rng = Rng::new(1);
        assert_eq!(rng.next(), 16807);
        assert_eq!(rng.next(), 282475249);
        assert_eq!(rng.next(), 1622650073);
    }

    #[test]
    fn generate_floats() {
        let mut rng = Rng::new(1);
        assert_eq!(rng.next_float(), 0.000007825903601782307);
        assert_eq!(rng.next_float(), 0.13153778773875702);
        assert_eq!(rng.next_float(), 0.7556053220812281);
    }

    #[test]
    fn shuffle_arrays() {
        let mut rng = Rng::new(1);
        let mut a = [0, 1, 2, 3, 4, 5, 6, 7];
        rng.shuffle(&mut a);
        assert_eq!(a, [1, 3, 6, 5, 2, 4, 7, 0]);
        rng.shuffle(&mut a);
        assert_eq!(a, [7, 1, 5, 6, 3, 0, 2, 4]);
        rng.shuffle(&mut a);
        assert_eq!(a, [2, 5, 0, 1, 4, 3, 6, 7]);
    }
}
