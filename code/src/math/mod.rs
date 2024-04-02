
fn calculate_fibonacci(number: i64) -> i128 {
    if number == 0 { return 0 }
    if number == 1 { return 1 }
    return calculate_fibonacci(number-1) + calculate_fibonacci(number-2)
}

pub fn fibonacci(number: i64) {
    let value: i128 = calculate_fibonacci(number);
    println!("Value of {} is {}", number, value);
}

fn calculate_factorial(number: i64) -> i128 {
    if number == 1 {
        return 1
    }
    return number as i128 * calculate_factorial((number-1) as i64)
}

pub fn factorial(number: i64) {
    let value = calculate_factorial(number);
    println!("Value of {} is {}", number, value);
}