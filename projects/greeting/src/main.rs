use std::process::exit;


pub mod method1 {
    use std::io::{self, Write};

    pub fn greeting() {

        print!("Enter you name: ");

        // consuming the result
        match io::stdout().flush() {
            Ok(_) => print!(""),
            Err(error) => println!("\n{error}")
        };
        
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line.");
    
        println!("Hello {name}");
    }
}



pub mod method2 {
    use std::io::{self, Write};

    pub fn greeting() {

        print!("Enter you name: ");

        // consuming the result
        io::stdout().flush().expect("Unable to flush");
        
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line.");
    
        println!("Hello {name}");
    }
}

/// Conditional greeting
/// Greeting only Alice and Bob
pub mod method3 {
    use std::io::{self, Write};

    pub fn greeting() {

        print!("Enter you name [Alice/Bob]: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let alice = "alice".to_lowercase().to_string();
        let bob: String = "bob".to_string().to_lowercase();
        let mut name = String::new();

        io::stdin().read_line(&mut name).expect("Unable to read name");
        name = name.trim().to_string();

        if name.trim() == bob {
            println!("Hello {name}");
        } else if name.eq(&alice) {
            println!("Hello {name}");
        } else {
            println!("You are not welcome here :(");
        }

    }
}

fn main() {
    method3::greeting();
    exit(0);
}
