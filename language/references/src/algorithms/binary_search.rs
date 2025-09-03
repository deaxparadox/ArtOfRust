pub mod binary_searh {
    fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
                std::cmp::Ordering::Equal => return Some(mid)
            }
        }
        None
    }
}