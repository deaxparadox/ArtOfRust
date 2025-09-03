use std::{
    io::Error,
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering}
};

use signal_hook::consts;
use signal_hook::flag;

mod custom_iterator;
mod algorithms;

mod element_in_vector {
    fn find_element<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &'static str> {
        for (index, element) in arr.iter().enumerate() {
            if *element == target {
                return Ok(index);
            }
        }
        Err("Element not found")
    }

    pub fn main() {
        let numbers: Vec<i32> = vec![10, 20, 30, 40, 50];

        match find_element(&numbers, 30) {
            Ok(index) => println!("Elment found at index: {}", index),
            Err(e) => println!("Error: {}", e)
        }

        match find_element(&numbers, 60) {
            Ok(index) => println!("Elment found at index: {}", index),
            Err(e) => println!("Error: {}", e)
        }
    }  
}

///
/// This function return the sum of 
/// square of number in vec which are 
/// not divisible by 2
fn higher_order_function() {
    let numbers: Vec<i32> = vec![1, 32, 4, 5, 6];

    let sum_of_numbers: i32 = numbers
        .into_iter()
        .filter(|&x| x % 2 != 0)
        .map(|x| x * x)
        .sum();

    println!("Sum of all the number is: {}", sum_of_numbers);
}

fn main() -> Result<(), Error>{
    
    algorithms::quick_sort::quick_sort::main();


    Ok(())
}

