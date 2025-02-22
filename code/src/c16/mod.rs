pub mod ex1_thread_usage;
pub mod ex2_join_threads;
pub mod ex5_closure_move;
pub mod ex7_mpsc;


pub mod ex8 {
    use std::sync::mpsc;
    use std::thread;

    pub fn first() {
        let (tx, rx) = mpsc::channel();


        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
}

// pub mod ex9 {
//     use std::sync::mpsc;
//     use std::thread;

//     fn main() {
//         let (tx, rx) = mpsc::channel();

//         thread::spawn(move || {
//             let val = String::from("hi");
//             tx.send(val).unwrap();
//             println!("val is {val}");
//         });

//         let received = rx.recv().unwrap();
//         println!("Got: {received}");
//     }
// }


pub mod ex10 {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn first() {
        let (tx, rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
}


/// trying `try_recv` method
pub mod myex1 {
    

    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn first() {
        let (tx, rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        loop {
            let value = rx.try_recv();

            match value {
                Ok(value) => println!("Got: {value}"),
                Err(value) => println!("No value received: {value}")
            }

            if t.is_finished() {
                t.join();
                break;
            }

            thread::sleep(Duration::from_millis(3000));
        }
    }
}


/// putting main thread in sleep.
/// and checking if we get all messages,
/// of some messages.
pub mod myex2 {
    use std::sync::mpsc;
    use std::thread;

    pub fn first() {
        let (tx, rx) = mpsc::channel();


        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
}
