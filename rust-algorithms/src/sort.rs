use std::{fmt::Debug, sync::Mutex};

use crate::rand;

pub trait Sort<T: PartialOrd + Debug> {
    fn bubble_sort(&mut self);
    fn quick_sort(&mut self);
}

pub trait MergeSort<T: PartialOrd + Debug> {
    fn merge_sort(self) -> Vec<T>;
}

impl<T: PartialOrd + Debug> Sort<T> for &mut [T] {
    fn bubble_sort(&mut self) {
        let len = self.len();

        for p in 0..len {
            println!("{:?}", self);

            let mut sorted = true;

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

    fn quick_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        println!("{:?}", self);

        let p = pivot(self);
        let (mut left, right) = self.split_at_mut(p);

        left.quick_sort();
        (&mut right[1..]).quick_sort(); // the middle element is already sorted
    }
}

fn pivot<T: PartialOrd>(data: &mut [T]) -> usize {
    let r = rand::rand_gen(data.len());
    data.swap(r, 0);

    let mut p = 0;

    for i in 1..data.len() {
        if data[i] < data[p] {
            data.swap(p + 1, i);
            data.swap(p, p + 1);
            p += 1;
        }
    }

    p
}

impl<T: PartialOrd + Debug> MergeSort<T> for Vec<T> {
    fn merge_sort(self) -> Vec<T> {
        merge_sort_handle(self)
    }
}

fn merge_sort_handle<T: PartialOrd + Debug>(mut data: Vec<T>) -> Vec<T> {
    let len = data.len();

    if len <= 1 {
        return data;
    }

    println!("{:?}", &data);

    // recurse
    let right = data.split_off(len / 2);
    let left = merge_sort_handle(data);
    let right = merge_sort_handle(right);

    // merge
    let mut result = Vec::with_capacity(len / 2 + 1);
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();

    loop {
        match left_peek {
            Some(ref left) => match right_peek {
                None => {
                    result.push(left_peek.take().unwrap());
                    result.extend(left_iter);
                    return result;
                }
                Some(ref right) => {
                    if left <= right {
                        result.push(left_peek.take().unwrap());
                        left_peek = left_iter.next();
                    } else {
                        result.push(right_peek.take().unwrap());
                        right_peek = right_iter.next();
                    }
                }
            },
            None => {
                if let Some(val) = right_peek {
                    result.push(val);
                }
                result.extend(right_iter);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bubble_sort() {
        // let mut data = vec![4, 6, 1, 8, 11, 13, 3];
        let mut data = vec![1, 3, 4, 6, 8, 11, 13];
        data.as_mut_slice().bubble_sort();

        assert_eq!(data, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_merge_sort() {
        let data = vec![4, 6, 1, 8, 11, 13, 3];
        let data = data.merge_sort();

        assert_eq!(data, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_pivot() {
        let mut data = vec![4, 0, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut data);

        for i in 0..data.len() {
            assert!((data[i] < data[p]) == (i < p))
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut data = vec![4, 0, 6, 1, 8, 11, 13, 3];
        data.as_mut_slice().quick_sort();

        assert_eq!(data, vec![0, 1, 3, 4, 6, 8, 11, 13])
    }
}
