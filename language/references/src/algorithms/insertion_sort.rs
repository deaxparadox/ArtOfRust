pub mod insertion_sort_1 {
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut j = i;
            while j > 0 && arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                j -= 1
            }
        }
    }
    pub fn main() {
        let mut arr = vec![23, 4, 65, 55, 98, 1];
        insertion_sort(&mut arr);
        print!("Sorted arr: {:?}", arr);
    }
}