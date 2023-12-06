use std::fmt::Debug;

pub trait Sort<T: PartialOrd + Debug> {
    fn bubble(&mut self);
}

impl<T: PartialOrd + Debug> Sort<T> for &mut [T] {
    fn bubble(&mut self) {
        let len = self.len();

        let mut sorted = true;

        for p in 0..len {
            println!("{:?}", &self);

            for i in 0..len - 1 - p {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    sorted = false;
                }
            }

            if sorted {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sort;

    #[test]
    fn test_bubble_sort() {
        // let mut data = vec![4, 6, 1, 8, 11, 13, 3];
        let mut data = vec![1, 3, 4, 6, 8, 11, 13];
        data.as_mut_slice().bubble();

        assert_eq!(data, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
