
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


pub mod quick_sort {
    fn quick_sort<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
        if low < high {
            let pivot_index = partition(arr, low, high);
            if pivot_index > 0 {
                // Avoid overflow in case of very small slices
                quick_sort(arr, low, pivot_index.saturating_sub(1));
            }
            quick_sort(arr, pivot_index + 1, high);
        }
    }

    fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
        let pivot = arr[high].clone();
        let mut i = low as isize - 1;

        let mut value: T;
        for j in low..high {
            value = arr[j].clone();
            if value <= pivot {
                i += 1;
                arr.swap(i as usize, j);
            }
        }

        arr.swap((i+1) as usize, high);
        (i + 1) as usize
    }

    pub fn main() {
        let mut data = [5, 3, 8, 6, 2, 7, 1, 4];
        let high = data.len();
        quick_sort(&mut data, 0,  high- 1);
        print!("{:?}", data);
    }
}