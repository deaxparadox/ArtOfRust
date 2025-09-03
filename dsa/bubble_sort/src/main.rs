
fn generic_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 {
            if arr[i] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers: Vec<i32> = vec![3, 5, 1, 4, 2];
    generic_sort(&mut numbers);
    print!("{:?}", numbers);

    println!("");

    let mut words: Vec<&'static str> = vec!["hello", "how", "azmuth"];
    generic_sort(&mut words);
    print!("{:?}", words);
}
