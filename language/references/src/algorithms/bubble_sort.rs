
pub mod buble_sort_1 {
    /// This module sort by ordering high element 
    fn buble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        let mut swapped: bool = false;

        for i in 0..len {
            swapped = false;
            for j in 0..(len-1-i) {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }

    pub fn main() {
        let mut arr = vec![98, 23, 43, 45, 12, 6, 3, 76];
        buble_sort(&mut arr);
        println!("Sorted arr: {:?}", arr);
    }
}
