pub mod method1 {
    use std::io::{self, Write};

    pub fn main() {
        let mut num_string = String::new();

        print!("Enter the your number: ");
        io::stdout().flush().expect("Error while flushing.");

        io::stdin().read_line(&mut num_string).expect("Unable to read number.");

        let num = num_string.trim().parse::<u32>().unwrap();
        let mut sum = 0;

        for i in 0..=num {
            sum+=i;
        }
        
        println!("Total sum {num} of: {sum}");
    }
}

pub mod method2 {
    use std::io::{self, Write};

    pub fn main() {
        let mut num_string = String::new();

        print!("Enter the your number: ");
        io::stdout().flush().expect("Error while flushing.");

        io::stdin().read_line(&mut num_string).expect("Unable to read number.");

        let num = num_string.trim().parse::<u32>().unwrap();
        let mut sum = 0;

        for i in 0..=num {
            if i % 3 == 0 {
                sum += i;
            } else if i % 5 == 0 {
                sum += i;
            }
        }
        
        println!("Total sum {num} of: {sum}");
    }
}

fn main() {
    method2::main();
}
