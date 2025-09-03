pub mod parallel_merge_sort_thread {
    use rand::prelude::*;
    use std::{thread, vec};

    fn merge_sort_parallel<T: Ord + Clone + Send + 'static>(arr: &mut [T]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let (left, right) = arr.split_at_mut(mid);

        // Using .to_vec() is necessary for cloning an array because thread may outlive arr.
        let left_handler = {
            let left = left.to_vec();
            thread::spawn(move || {
                let mut left = left;
                merge_sort_parallel(&mut left);
                left
            })
        };

        let right_handler = {
            let right = right.to_vec();
            thread::spawn(move || {
                let mut right = right;
                merge_sort_parallel(&mut right);
                right
            })
        };

        let left_sorted = left_handler.join().unwrap();
        let right_sorted = right_handler.join().unwrap();

        merge(arr, &left_sorted, &right_sorted);
    }

    fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i].clone();
                i += 1
            } else {
                arr[k] = right[j].clone();
                j += 1
            }
            k += 1;
        }

        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }

    pub fn main() {
        // let mut arr = vec![23, 12, 56, 34, 923, 23, 43, 5, 23, 676, 3, 43];
        let mut arr = vec![100000];

        let mut rng = rand::rng();

        let mut arr: Vec<i32> = (1..100000).collect();
        arr.shuffle(&mut rng);

        merge_sort_parallel(&mut arr);
        println!("Sorted array: {:?}", arr);
    }
}

pub mod parallel_merge_sort_rayon {
    use rayon::prelude::*;

    pub fn parallel_merge_sort<T: Ord + Send + Sync + Clone>(arr: &mut [T]) {
        if arr.len() <= 1{
            // Base case: array of length 0 or 1 is already sorted
            return;
        }

        let mid = arr.len() / 2;
        let len = arr.len();
        let (left, right) = arr.split_at_mut(mid);

        // Sort the halves in parallel
        rayon::join(
            || parallel_merge_sort(left),
            || parallel_merge_sort(right),
        );

        let mut sorted = Vec::with_capacity(len);
        merge(left, right, &mut sorted);

        // Copy sorted elements back to original array
        for (i, elem) in sorted.into_iter().enumerate() {
            arr[i] = elem;
        }
    }

    pub fn merge<T: Ord + Clone>(left: &[T], right: &[T], merged: &mut Vec<T>) {
        let mut left_iter = left.iter();
        let mut right_iter = right.iter();

        let mut left_val = left_iter.next();
        let mut right_val = right_iter.next();

        while left_val.is_some() && right_val.is_some() {
            if left_val < right_val {
                merged.push(left_val.take().unwrap().clone());
                left_val = left_iter.next();
            } else {
                merged.push(right_val.take().unwrap().clone());
                right_val = right_iter.next()
            }
        }

        while let Some(val) = left_val {
            merged.push(val.clone());
            left_val = left_iter.next();
        }

        while let Some(val) = right_val {
            merged.push(val.clone());
            right_val = right_iter.next();
        }
    }
}
