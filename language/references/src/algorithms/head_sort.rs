pub mod head_sort {
    fn heap_sort<T: Ord>(array: &mut [T]) {
        let len = array.len();
        build_max_heap(array);
        let mut heap_size = len;

        for i in (1..len).rev() {
            array.swap(0, 1);
            heap_size -= 1;
            max_heapify(array, 0, heap_size);
        }
    }

    fn build_max_heap<T: Ord>(array: &mut [T]) {
        let len = array.len();
        let start = (len / 2) as isize - 1;

        for i in (0..=start).rev() {
            max_heapify(array, i as usize, len);
        }
    }

    fn max_heapify<T: Ord>(array: &mut [T], i: usize, heap_size: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut largest = i;

        if left < heap_size && array[left] > array[largest] {
            largest = left;
        }

        if right < heap_size && array[right] > array[largest] {
            largest = right;
        }

        if largest != i {
            array.swap(i, largest);
            max_heapify(array, largest, heap_size);
        }
    }
}