use std::io::{self, Write};

/// Read the input, compute it.
/// 
pub fn main() {
    // 
    print!("Enter your favourite number: ");
    io::stdout().flush().expect("Unable to flush buffer");
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).expect("Unable to read number.");
    
    print!("Enter your computing method [sum/mul]: ");
    io::stdout().flush().expect("Unable to flush buffer");
    let mut input_method = String::new();
    io::stdin().read_line(&mut input_method).expect("Unable to read calculate method");
    input_method = input_method.trim().to_lowercase();

    // Convert it num
    let user_num = input_num.trim().parse::<u32>().unwrap();


    let mut sum: u64 = 0;
    if input_method == "sum".to_string() {
        for i in 0..=user_num {
            sum+=i as u64;
        }
        println!("Calculated sum is: {sum}");
    } else if input_method == "mul".to_string() {
        sum = 1;
        for i in 1..=user_num {
            sum*=i as u64;
        }
        println!("Calculated product is: {sum}");
    } else {
        panic!("Incorrect computing method");
    }

}