pub mod selection_sort_1 {
    fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();

        for i in 0..len {
            let mut min_index = i;
            for j in (min_index+1)..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                arr.swap(min_index, i);
            }
        }
    }

    pub fn main() {
        let mut arr = vec![23, 43, 45, 12, 6, 3, 76];
        selection_sort(&mut arr);
        println!("Sorted arr: {:?}", arr);
    }
}