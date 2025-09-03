
pub mod quick_sort_high {
    pub fn main() {
        let mut arr = vec![23, 4, 1, 4, 83, 43, 64, 29, 98, 100];
        quick_sort(&mut arr);
        println!("Sorted array: {:?}", arr);
    }

    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        quick_sort(&mut left[0..pivot_index]);
        quick_sort(&mut right[1..]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len / 2;
        arr.swap(pivot_index, len-1);

        let mut i = 0;

        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, len - 1);
        i
    }   
}